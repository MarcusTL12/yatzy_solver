use std::{
    fs::OpenOptions,
    io::{stdin, stdout, Read, Seek, SeekFrom, Write},
};

use crate::{
    dice_distributions::amt_dice_combinations,
    dice_throw::DiceThrow,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
    macrosolver::outcore::Layer,
    yatzy::{cell_from_dice, State},
};

const HELP_MSG: &str = r#"
commands:
help: displays this message
help cell names: display the name legend for cells
exit/q: exit
display points: display your points
set points <cell> <points>: set a cell to a value. Get cell names by
    help cell names
put dice <cell>: put the current dice into the cell.
clear points <cell>: clears points
advise <dice-left> <dice>: gives advice on what to do with the dice
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
        Some(x) if x == 0 => "-".to_owned(),
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

fn get_yatzy_index<const N: usize>(name: &str) -> usize {
    match (N, name) {
        (_, "1s") => 0,
        (_, "2s") => 1,
        (_, "3s") => 2,
        (_, "4s") => 3,
        (_, "5s") => 4,
        (_, "6s") => 5,
        (_, "1p") => 6,
        (_, "2p") => 7,
        (6, "3p") => 8,
        (5, "3l") => 8,
        (6, "3l") => 9,
        (5, "4l") => 9,
        (6, "4l") => 10,
        (6, "5l") => 11,
        (5, "ls") => 10,
        (6, "ls") => 12,
        (5, "ss") => 11,
        (6, "ss") => 13,
        (6, "fs") => 14,
        (5, "hs") => 12,
        (6, "ht") => 15,
        (6, "hs") => 16,
        (6, "tr") => 17,
        (5, "ch" | "sj") => 13,
        (6, "ch" | "sj") => 18,
        (5, "yz") => 14,
        (6, "yz") => 19,
        _ => unreachable!(),
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

fn get_byte_from_file(filename: &str, index: usize) -> u8 {
    let mut file = OpenOptions::new().read(true).open(filename).unwrap();

    file.seek(SeekFrom::Start(index as u64)).unwrap();

    let mut buf = [0];

    file.read_exact(&mut buf).unwrap();

    buf[0]
}

fn get_float_from_file(filename: &str, index: usize) -> f32 {
    let mut file = OpenOptions::new().read(true).open(filename).unwrap();

    file.seek(SeekFrom::Start((index * 4) as u64)).unwrap();

    let mut buf = [0; 4];

    file.read_exact(&mut buf).unwrap();

    f32::from_le_bytes(buf)
}

fn get_cell_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
) -> usize
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

    let layer = Layer::<N> {
        na,
        nb,
        nt: 0,
        scores: None,
        strats: None,
    };

    get_byte_from_file(&layer.strats_path(), total_index) as usize
}

fn get_rethrow_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    throws_left: usize,
    points_above: usize,
) -> u8 {
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N> {
        na,
        nb,
        nt: throws_left,
        scores: None,
        strats: None,
    };

    get_byte_from_file(&layer.strats_path(), total_index)
}

fn get_score<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
    throws_left: usize,
) -> f32 {
    let [na, nb, _, lb, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let lt = amt_dice_combinations::<N>();
    let ti = dice.get_index();

    let total_index = (ai * lb + bi) * lt + ti;

    let layer = Layer::<N> {
        na,
        nb,
        nt: throws_left,
        scores: None,
        strats: None,
    };

    get_float_from_file(&layer.scores_path(), total_index)
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

pub fn start<const N: usize>()
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
    println!("Throws left: {throws_left}");

    'outer: loop {
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
                let index = get_yatzy_index::<N>(cell);
                let pts = pts.parse().unwrap();
                points[index] = Some(pts);
                display_points::<N>(&points, None, None);
            }
            ["put", "dice", cell] => {
                let index = get_yatzy_index::<N>(cell);
                let pts = dice.cell_score::<N>(index);
                points[index] = Some(pts);
                display_points::<N>(&points, None, None);
                throws_left = 2;
                dice = DiceThrow::throw(N);
                println!("Current dice:\n{}", dice);
                println!("Throws left: {throws_left}");
            }
            ["clear", "points", cell] => {
                let index = get_yatzy_index::<N>(cell);

                points[index] = None;
            }
            ["throw", n] => {
                let n = n.parse().unwrap();
                let throw = DiceThrow::throw(n);
                println!("{}", throw);
                dice = throw;
            }
            ["auto"] | [] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                if throws_left == 0 {
                    let ind =
                        get_cell_strat::<N>(&filled_cells, &dice, points_above);

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
                } else {
                    let reroll = get_rethrow_strat::<N>(
                        &filled_cells,
                        &dice,
                        throws_left,
                        points_above,
                    );

                    println!("Rethrowing:\n{}", dice.get_subthrow(reroll));

                    let rethrow =
                        DiceThrow::throw(reroll.count_ones() as usize);

                    dice = dice.overwrite_reroll_dyn::<N>(
                        reroll,
                        &rethrow.into_ordered_dice().collect::<Vec<_>>(),
                    );

                    println!("To give:\n{}", dice);
                    throws_left -= 1;
                }

                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                let rem_score = get_score::<N>(
                    &filled_cells,
                    &dice,
                    points_above,
                    throws_left,
                );

                let tot_score =
                    get_total_score::<N>(&points) as f32 + rem_score;

                println!("expected total score is now {:.2}", tot_score);
            }
            ["advise" | "a", dice_left, dice_str] => {
                let throws_left: usize = dice_left.parse().unwrap();
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

                match throws_left {
                    0 => {
                        let ind = get_cell_strat::<N>(
                            &filled_cells,
                            &throw,
                            points_above,
                        );

                        let score = throw.cell_score::<N>(ind);

                        println!(
                            "Put {} points in {}.",
                            score,
                            get_index_name::<N>(ind)
                        );
                    }
                    1 | 2 => {
                        let reroll = get_rethrow_strat::<N>(
                            &filled_cells,
                            &throw,
                            throws_left,
                            points_above,
                        );

                        println!("Rethrow:\n{}", throw.get_subthrow(reroll));
                    }
                    _ => unreachable!(),
                }
            }
            ["expected-remaining" | "ex-r"] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();
                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();
                let rem_score = get_score::<N>(
                    &filled_cells,
                    &dice,
                    points_above,
                    throws_left,
                );

                println!("expected remaining score is {}", rem_score);
            }
            ["expected-total" | "ex-t"] => {
                let filled_cells: Vec<_> =
                    points.iter().map(|x| x.is_some()).collect();
                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();
                let rem_score = get_score::<N>(
                    &filled_cells,
                    &dice,
                    points_above,
                    throws_left,
                );

                let tot_score =
                    get_total_score::<N>(&points) as f32 + rem_score;

                println!("expected total score is {}", tot_score);
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
                println!("Throws left: {throws_left}");
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
