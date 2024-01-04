use std::{
    fs::OpenOptions,
    io::{stdin, stdout, Read, Seek, SeekFrom, Write},
    process::{Command, Stdio},
    time::Instant,
};

use arrayvec::ArrayVec;

use crate::{
    dice_distributions::amt_dice_combinations,
    dice_throw::DiceThrow,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
    macrosolver::outcore::{Layer, PREFIX},
    yatzy::{cell_from_dice, State},
};

const HELP_MSG: &str = r#"
commands:
help: displays this message
help cell names: display the name legend for cells
exit/q: exit
display points: display your points
set points/sp <cell> <points>: set a cell to a value. Get cell names by
    help cell names
put dice <cell>: put the current dice into the cell.
clear points <cell>: clears points
advise/a <dice-left> <dice>: gives advice on what to do with the dice
throw <N>: prints a dice throw of <N> dice
auto: automatically perform the next optimal move
"#;

const HELP_CELL_NAMES: &str = r#"
ones/enere...       => 1s - 6s
pairs/par           => 1p - 3p
of a kind / like    => 1l - 6l
straight            => ls, ss, fs
hut/hytte           => ht
house/hus           => hs
tower/tårn          => tr
chance/sjangse      => ch/sj
yatzy               => yz
"#;

fn tostr(points: &[Option<usize>], ind: &mut usize) -> String {
    let ans = match points[*ind] {
        None => "".to_owned(),
        Some(0) => "-".to_owned(),
        Some(x) => format!("{}", x),
    };
    *ind += 1;
    ans
}

fn display_points<const N: usize>(
    points: &[Option<usize>],
    prec_bonus: Option<usize>,
    prec_sum: Option<usize>,
) {
    let ind = &mut 0;
    println!("ones              (1s) = {}", tostr(points, ind));
    println!("twos              (2s) = {}", tostr(points, ind));
    println!("threes            (3s) = {}", tostr(points, ind));
    println!("fours             (4s) = {}", tostr(points, ind));
    println!("fives             (5s) = {}", tostr(points, ind));
    println!("sixes             (6s) = {}", tostr(points, ind));
    println!("------------------------------------");
    let above: usize = points.iter().take(6).filter_map(|x| x.as_ref()).sum();
    let bonus_objective = match N {
        5 => 63,
        6 => 84,
        _ => unreachable!(),
    };
    let bonus_objective: usize = (0..bonus_objective).map(|_| 1).sum();

    let bonus = if above >= bonus_objective {
        match N {
            5 => 50,
            6 => 100,
            _ => unreachable!(),
        }
    } else {
        0
    };

    let bonus: usize = if let Some(b) = prec_bonus {
        b
    } else {
        (0..bonus).map(|_| 1).sum()
    };

    println!("sum                    = {}", above);
    println!("bonus                  = {}", bonus);
    println!("1 pair            (1p) = {}", tostr(points, ind));
    println!("2 pair            (2p) = {}", tostr(points, ind));
    if N == 6 {
        println!("3 pair            (3p) = {}", tostr(points, ind));
    }
    println!("3 of a kind       (3l) = {}", tostr(points, ind));
    println!("4 of a kind       (4l) = {}", tostr(points, ind));
    if N == 6 {
        println!("5 of a kind       (5l) = {}", tostr(points, ind));
    }
    println!("small straight    (ls) = {}", tostr(points, ind));
    println!("large straight    (ss) = {}", tostr(points, ind));
    if N == 6 {
        println!("full straight     (fs) = {}", tostr(points, ind));
        println!("hut               (ht) = {}", tostr(points, ind));
    }
    println!("house             (hs) = {}", tostr(points, ind));
    if N == 6 {
        println!("tower             (tr) = {}", tostr(points, ind));
    }
    println!("chance         (sj/ch) = {}", tostr(points, ind));
    println!("yatzy             (yz) = {}", tostr(points, ind));
    println!("------------------------------------");
    println!(
        "Total                  = {}\n",
        if let Some(s) = prec_sum {
            s
        } else {
            bonus + points.iter().filter_map(|x| x.as_ref()).sum::<usize>()
        }
    );
}

fn get_yatzy_index<const N: usize>(name: &str) -> Option<usize> {
    match (N, name) {
        (_, "1s") => Some(0),
        (_, "2s") => Some(1),
        (_, "3s") => Some(2),
        (_, "4s") => Some(3),
        (_, "5s") => Some(4),
        (_, "6s") => Some(5),
        (_, "1p") => Some(6),
        (_, "2p") => Some(7),
        (6, "3p") => Some(8),
        (5, "3l") => Some(8),
        (6, "3l") => Some(9),
        (5, "4l") => Some(9),
        (6, "4l") => Some(10),
        (6, "5l") => Some(11),
        (5, "ls") => Some(10),
        (6, "ls") => Some(12),
        (5, "ss") => Some(11),
        (6, "ss") => Some(13),
        (6, "fs") => Some(14),
        (5, "hs") => Some(12),
        (6, "ht") => Some(15),
        (6, "hs") => Some(16),
        (6, "tr") => Some(17),
        (5, "ch" | "sj") => Some(13),
        (6, "ch" | "sj") => Some(18),
        (5, "yz") => Some(14),
        (6, "yz") => Some(19),
        _ => None,
    }
}

fn get_index_name<const N: usize>(ind: usize) -> &'static str {
    match (N, ind) {
        (_, 0) => "ones",
        (_, 1) => "twos",
        (_, 2) => "threes",
        (_, 3) => "fours",
        (_, 4) => "fives",
        (_, 5) => "sixes",
        (_, 6) => "1 pair",
        (_, 7) => "2 pairs",
        (6, 8) => "3 pairs",
        (5, 8) | (6, 9) => "3 of a kind",
        (5, 9) | (6, 10) => "4 of a kind",
        (6, 11) => "5 of a kind",
        (5, 10) | (6, 12) => "small straight",
        (5, 11) | (6, 13) => "large straight",
        (6, 14) => "full straight",
        (6, 15) => "hut",
        (5, 12) | (6, 16) => "house",
        (6, 17) => "tower",
        (5, 13) | (6, 18) => "chance",
        (5, 14) | (6, 19) => "yatzy",
        _ => unreachable!(),
    }
}

pub fn get_state_indices5(cells: &[bool], points_above: usize) -> [usize; 6] {
    let state = State::<15>::from_dyn(cells, points_above);

    let na = state.get_n_above();
    let nb = state.get_n_below();

    let la = ABOVE_LEVELS_5[na].len();
    let lb = BELOW_LEVELS_5[nb].len();

    let ai = state.get_above_index();
    let bi = state.get_below_index();

    [na, nb, la, lb, ai, bi]
}

pub fn get_state_indices6(cells: &[bool], points_above: usize) -> [usize; 6] {
    let state = State::<20>::from_dyn(cells, points_above);

    let na = state.get_n_above();
    let nb = state.get_n_below();

    let la = ABOVE_LEVELS_6[na].len();
    let lb = BELOW_LEVELS_6[nb].len();

    let ai = state.get_above_index();
    let bi = state.get_below_index();

    [na, nb, la, lb, ai, bi]
}

fn get_byte_from_file(filename: &str, index: usize) -> Option<u8> {
    let mut file = OpenOptions::new().read(true).open(filename).ok()?;

    file.seek(SeekFrom::Start(index as u64)).ok()?;

    let mut buf = [0];

    file.read_exact(&mut buf).ok()?;

    Some(buf[0])
}

fn get_float_from_file(filename: &str, index: usize) -> Option<f32> {
    let mut file = OpenOptions::new().read(true).open(filename).ok()?;

    file.seek(SeekFrom::Start((index * 4) as u64)).ok()?;

    let mut buf = [0; 4];

    file.read_exact(&mut buf).ok()?;

    Some(f32::from_le_bytes(buf))
}

fn get_bytes_from_compressed_file<
    const N: usize,
    const X: bool,
    const M: usize,
>(
    na: usize,
    nb: usize,
    nt: usize,
    pref_str: &str,
    index: usize,
) -> Option<[u8; M]> {
    let typename = match (N, X) {
        (5, false) => "5",
        (6, false) => "6",
        (5, true) => "5x",
        (6, true) => "6x",
        _ => panic!(),
    };
    let archive_name = format!("{pref_str}.7z");
    let path = format!("{}/{typename}/{archive_name}", PREFIX.as_str());
    let internal_path = format!("{pref_str}/{na}_{nb}_{nt}.dat");

    println!("Decompressing from archive: ");
    let timer = Instant::now();

    let mut proc = Command::new("7z")
        .args(["e", &path, &internal_path, "-so"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let ret = proc
        .stdout
        .take()
        .unwrap()
        .bytes()
        .filter_map(|x| x.ok())
        .skip(index)
        .take(M)
        .collect::<ArrayVec<_, M>>()
        .into_inner()
        .ok();

    proc.kill().unwrap();

    if ret.is_some() {
        println!("took {:.2?}", timer.elapsed());
    } else {
        println!("could not get data from archive")
    }

    ret
}

fn get_byte_from_compressed_strats<const N: usize, const X: bool>(
    na: usize,
    nb: usize,
    nt: usize,
    index: usize,
) -> Option<u8> {
    get_bytes_from_compressed_file::<N, X, 1>(na, nb, nt, "strats", index)
        .map(|bytes| bytes[0])
}

fn get_float_from_compressed_scores<const N: usize, const X: bool>(
    na: usize,
    nb: usize,
    nt: usize,
    index: usize,
) -> Option<f32> {
    get_bytes_from_compressed_file::<N, X, 4>(na, nb, nt, "scores", index * 4)
        .map(f32::from_le_bytes)
}

fn get_cell_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
) -> Option<usize>
where
    [(); cell_from_dice::<N>()]:,
    [(); cell_from_dice::<N>() - 6]:,
{
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N, false> {
        na,
        nb,
        nt: 0,
        scores: None,
        strats: None,
    };

    if let Some(x) = get_byte_from_file(&layer.strats_path(), total_index) {
        Some(x)
    } else {
        get_byte_from_compressed_strats::<N, false>(na, nb, 0, total_index)
    }
    .map(|x| x as usize)
}

fn get_rethrow_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    throws_left: usize,
    points_above: usize,
) -> Option<u8> {
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N, false> {
        na,
        nb,
        nt: throws_left,
        scores: None,
        strats: None,
    };

    if let Some(x) = get_byte_from_file(&layer.strats_path(), total_index) {
        Some(x)
    } else {
        get_byte_from_compressed_strats::<N, false>(
            na,
            nb,
            throws_left,
            total_index,
        )
    }
}

pub enum Strategy {
    Rethrow(u8),
    Cell(usize),
}

fn get_combined_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    throws_left: usize,
    points_above: usize,
) -> Option<Strategy> {
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N, true> {
        na,
        nb,
        nt: throws_left,
        scores: None,
        strats: None,
    };

    let byte = if let Some(x) =
        get_byte_from_file(&layer.strats_path(), total_index)
    {
        Some(x)
    } else {
        get_byte_from_compressed_strats::<N, true>(
            na,
            nb,
            throws_left,
            total_index,
        )
    };

    byte.map(|byte| {
        if (byte & 128) != 0 {
            Strategy::Rethrow(byte & !128)
        } else {
            Strategy::Cell(byte as usize)
        }
    })
}

fn get_score<const N: usize, const X: bool>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
    throws_left: usize,
) -> Option<f32> {
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N, X> {
        na,
        nb,
        nt: throws_left,
        scores: None,
        strats: None,
    };

    if let Some(x) = get_float_from_file(&layer.scores_path(), total_index) {
        Some(x)
    } else {
        get_float_from_compressed_scores::<N, X>(
            na,
            nb,
            throws_left,
            total_index,
        )
    }
}

pub fn get_total_score<const N: usize>(points: &[Option<usize>]) -> usize {
    let points_above: usize =
        points.iter().take(6).filter_map(|x| x.as_ref()).sum();

    let bonus_objective = match N {
        5 => 63,
        6 => 84,
        _ => unreachable!(),
    };

    let bonus = if points_above >= bonus_objective {
        match N {
            5 => 50,
            6 => 100,
            _ => unreachable!(),
        }
    } else {
        0
    };

    let total = bonus + points.iter().filter_map(|x| x.as_ref()).sum::<usize>();

    total
}

pub fn start<const N: usize, const X: bool>()
where
    [(); cell_from_dice::<N>()]:,
    [(); cell_from_dice::<N>() - 6]:,
{
    println!(
        "Welcome to the interactive guide of a free game with {} dice",
        N
    );

    let mut points = vec![
        None;
        match N {
            5 => 15,
            6 => 20,
            _ => unreachable!(),
        }
    ];

    let mut dice = DiceThrow::throw(N);
    let mut throws_left = 2;

    println!("Starting throw:\n{}", dice);

    let mut print_score = false;

    'outer: loop {
        println!("Throws left: {throws_left}");

        let filled_cells: Vec<_> = points.iter().map(|x| x.is_some()).collect();
        let points_above =
            points.iter().take(6).filter_map(|x| x.as_ref()).sum();

        if print_score {
            if let Some(rem_score) = get_score::<N, X>(
                &filled_cells,
                &dice,
                points_above,
                throws_left,
            ) {
                let tot_score =
                    get_total_score::<N>(&points) as f32 + rem_score;

                println!("expected total score is now {:.2}", tot_score);
            }
        }

        print_score = true;

        print!("> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let command: Vec<_> = buffer.split_whitespace().collect();

        match command.as_slice() {
            ["help"] => println!("{}", HELP_MSG),
            ["help", "cell", "names"] => println!("{}", HELP_CELL_NAMES),
            ["exit" | "q"] => break,
            ["display", "points"] => display_points::<N>(&points, None, None),
            ["set", "points", cell, pts] | ["sp", cell, pts] => {
                if let Some(index) = get_yatzy_index::<N>(cell) {
                    let pts = pts.parse().unwrap();
                    points[index] = Some(pts);
                    display_points::<N>(&points, None, None);
                } else {
                    println!("Invalid cell name!");
                }

                print_score = false;
            }
            ["put", "dice", cell] => {
                if let Some(index) = get_yatzy_index::<N>(cell) {
                    let pts = dice.cell_score::<N>(index);
                    points[index] = Some(pts);
                    display_points::<N>(&points, None, None);
                    if X {
                        throws_left += 2;
                    } else {
                        throws_left = 2;
                    }
                    dice = DiceThrow::throw(N);
                    println!("Current dice:\n{}", dice);
                } else {
                    println!("Invalid cell name!");
                }
            }
            ["clear", "points", cell] => {
                if let Some(index) = get_yatzy_index::<N>(cell) {
                    points[index] = None;
                } else {
                    println!("Invalid cell name!");
                }
            }
            ["throw", n] => {
                let n = n.parse().unwrap();
                let throw = DiceThrow::throw(n);
                println!("{}", throw);
                dice = throw;
                print_score = false;
            }
            ["auto"] | [] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                if X {
                    match get_combined_strat::<N>(
                        &filled_cells,
                        &dice,
                        throws_left,
                        points_above,
                    ) {
                        Some(Strategy::Cell(ind)) => {
                            let score = dice.cell_score::<N>(ind);

                            println!(
                                "Putting {} points in {}.",
                                score,
                                get_index_name::<N>(ind)
                            );

                            points[ind] = Some(score);
                            display_points::<N>(&points, None, None);

                            dice = DiceThrow::throw(N);
                            throws_left += 2;

                            println!("New throw:\n{}", dice);
                        }
                        Some(Strategy::Rethrow(reroll)) => {
                            println!(
                                "Rethrowing:\n{}",
                                dice.get_subthrow(reroll)
                            );

                            let rethrow =
                                DiceThrow::throw(reroll.count_ones() as usize);

                            dice = dice.overwrite_reroll_dyn::<N>(
                                reroll,
                                &rethrow
                                    .into_ordered_dice()
                                    .collect::<Vec<_>>(),
                            );

                            println!("To give:\n{}", dice);
                            throws_left -= 1;
                        }
                        None => {
                            println!("Strategy not available for state.");
                        }
                    }
                } else if throws_left == 0 {
                    if let Some(ind) =
                        get_cell_strat::<N>(&filled_cells, &dice, points_above)
                    {
                        let score = dice.cell_score::<N>(ind);

                        println!(
                            "Putting {} points in {}.",
                            score,
                            get_index_name::<N>(ind)
                        );

                        points[ind] = Some(score);
                        display_points::<N>(&points, None, None);

                        dice = DiceThrow::throw(N);
                        throws_left = 2;

                        println!("New throw:\n{}", dice);
                    }
                } else if let Some(reroll) = get_rethrow_strat::<N>(
                    &filled_cells,
                    &dice,
                    throws_left,
                    points_above,
                ) {
                    println!("Rethrowing:\n{}", dice.get_subthrow(reroll));

                    let rethrow =
                        DiceThrow::throw(reroll.count_ones() as usize);

                    dice = dice.overwrite_reroll_dyn::<N>(
                        reroll,
                        &rethrow.into_ordered_dice().collect::<Vec<_>>(),
                    );

                    println!("To give:\n{}", dice);
                    throws_left -= 1;
                } else {
                    println!("Strategy not available for state.");
                }
            }
            ["advise" | "a", dice_left, dice_str] => {
                let throws_left_local: usize = if *dice_left == "x" {
                    throws_left
                } else {
                    dice_left.parse().unwrap()
                };
                if dice_str.len() != N {
                    continue 'outer;
                }
                let mut throw = DiceThrow::from([0usize; 6]);
                for c in dice_str.chars() {
                    let i = (c as u8 - b'0') as usize;
                    throw[i] += 1;
                }

                println!("You entered:\n{}\n", throw);

                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                match (throws_left_local, X) {
                    (0, false) => {
                        if let Some(ind) = get_cell_strat::<N>(
                            &filled_cells,
                            &throw,
                            points_above,
                        ) {
                            let score = throw.cell_score::<N>(ind);

                            println!(
                                "Put {} points in {}.",
                                score,
                                get_index_name::<N>(ind)
                            );
                        } else {
                            println!("Strategy not available for state.");
                        }
                    }
                    (1 | 2, false) => {
                        if let Some(reroll) = get_rethrow_strat::<N>(
                            &filled_cells,
                            &throw,
                            throws_left_local,
                            points_above,
                        ) {
                            println!(
                                "Rethrow:\n{}",
                                throw.get_subthrow(reroll)
                            );
                        } else {
                            println!("Strategy not available for state.");
                        }
                    }
                    (_, true) => match get_combined_strat::<N>(
                        &filled_cells,
                        &throw,
                        throws_left_local,
                        points_above,
                    ) {
                        Some(Strategy::Cell(ind)) => {
                            throws_left = throws_left_local + 2;

                            let score = throw.cell_score::<N>(ind);

                            println!(
                                "Put {} points in {}.",
                                score,
                                get_index_name::<N>(ind)
                            );
                        }
                        Some(Strategy::Rethrow(reroll)) => {
                            throws_left = throws_left_local - 1;
                            println!("Rethrow:\n{}", throw.get_subthrow(reroll))
                        }
                        None => println!("Strategy not available for state."),
                    },
                    _ => unreachable!(),
                }

                if let Some(rem_score) = get_score::<N, X>(
                    &filled_cells,
                    &throw,
                    points_above,
                    throws_left_local,
                ) {
                    let tot_score =
                        get_total_score::<N>(&points) as f32 + rem_score;

                    println!("expected total score is then {:.2}", tot_score);
                }

                print_score = false;
            }
            ["expected-remaining" | "ex-r"] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();
                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();
                if let Some(rem_score) = get_score::<N, X>(
                    &filled_cells,
                    &dice,
                    points_above,
                    throws_left,
                ) {
                    println!("expected remaining score is {}", rem_score);
                } else {
                    println!("Failed to read scores file");
                }
            }
            ["expected-total" | "ex-t"] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();
                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();
                if let Some(rem_score) = get_score::<N, X>(
                    &filled_cells,
                    &dice,
                    points_above,
                    throws_left,
                ) {
                    let tot_score =
                        get_total_score::<N>(&points) as f32 + rem_score;

                    println!("expected total score is {}", tot_score);
                } else {
                    println!("Failed to read scores file");
                }
            }
            ["reset"] => {
                points = vec![
                    None;
                    match N {
                        5 => 15,
                        6 => 20,
                        _ => unreachable!(),
                    }
                ];
                dice = DiceThrow::throw(N);
                throws_left = 2;

                println!("Starting throw:\n{}", dice);
            }
            ["rethrow" | "rt", mask_str] => {
                if throws_left == 0 {
                    println!("No throws left!");
                    continue 'outer;
                }

                if mask_str.len() != N {
                    println!("Invalid mask!");
                    continue 'outer;
                }

                let mut mask: u8 = 0;
                for c in mask_str.chars().rev() {
                    let bit = match c {
                        '0' => 0,
                        '1' => 1,
                        _ => {
                            println!("Invalid mask!");
                            continue 'outer;
                        }
                    };

                    mask = mask << 1 | bit;
                }

                let rethrow: Vec<_> =
                    DiceThrow::throw(mask.count_ones() as usize)
                        .into_ordered_dice()
                        .collect();

                dice = dice.overwrite_reroll_dyn::<N>(mask, &rethrow);
                throws_left -= 1;

                println!("New throw:\n{}", dice);
            }
            ["set", "dice", dice_str] => {
                if dice_str.len() != N {
                    continue 'outer;
                }
                let mut throw = DiceThrow::from([0usize; 6]);
                for c in dice_str.chars() {
                    let i = (c as u8 - b'0') as usize;
                    throw[i] += 1;
                }

                dice = throw;
                println!("Current dice:\n{}", dice);
            }
            _ => println!("Invalid command! {:?}", command),
        }
    }
}
