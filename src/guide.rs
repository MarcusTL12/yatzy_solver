use std::io::{stdin, stdout, Write};

use crate::dice_throw::DiceThrow;

const HELP_MSG: &str = r#"
commands:
help: displays this message
exit/q: exit
display points: display your points
set points <cell> <points>: set a cell to a value. Get cell names by
    help cell names
clear points <cell>: clears points
advise <dice-left> <dice>: gives advice on what to do with the dice
throw dice <N>: prints a dice throw of <N> dice
"#;

pub const HELP_CELL_NAMES: &str = r#"
ones/enere...       => 1s - 6s
pairs/par           => 1p - 3p
of a kind / like    => 1l - 5l
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
    println!("ones              = {}", tostr(points, ind));
    println!("twos              = {}", tostr(points, ind));
    println!("threes            = {}", tostr(points, ind));
    println!("fours             = {}", tostr(points, ind));
    println!("fives             = {}", tostr(points, ind));
    println!("sixes             = {}", tostr(points, ind));
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

    println!("sum               = {}", above);
    println!("bonus             = {}", bonus);
    println!("1 pair            = {}", tostr(points, ind));
    println!("2 pair            = {}", tostr(points, ind));
    if N == 6 {
        println!("3 pair            = {}", tostr(points, ind));
    }
    println!("3 of a kind       = {}", tostr(points, ind));
    println!("4 of a kind       = {}", tostr(points, ind));
    if N == 6 {
        println!("5 of a kind       = {}", tostr(points, ind));
    }
    println!("small straight    = {}", tostr(points, ind));
    println!("large straight    = {}", tostr(points, ind));
    if N == 6 {
        println!("full straight     = {}", tostr(points, ind));
        println!("hut               = {}", tostr(points, ind));
    }
    println!("house             = {}", tostr(points, ind));
    if N == 6 {
        println!("tower             = {}", tostr(points, ind));
    }
    println!("chance            = {}", tostr(points, ind));
    println!("yahtzee           = {}", tostr(points, ind));
    println!("------------------------------------");
    println!(
        "Total             = {}\n",
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
        (5, 14) | (6, 19) => "yahtzee",
        _ => unreachable!(),
    }
}

fn get_cell_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
) -> usize {
    todo!()
}

fn get_rethrow_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    throws_left: usize,
    points_above: usize,
) -> u8 {
    todo!()
}

fn new_throw(throw: &DiceThrow, mask: u8, rethrow: DiceThrow) -> DiceThrow {
    todo!()
}

// fn get_score<const N: usize>(
//     cells: &[bool],
//     dice: &DiceThrow,
//     points_above: u64,
//     throws_left: usize,
// ) -> f32 {
//     let free_cells = cells.iter().map(|&x| x).count();
//     let ans = {
//         Command::new("7z")
//             .arg("x")
//             .arg(Path::new(&*SCORES_PATH).join(format!("{}/scores.7z", N)))
//             .arg(format!(
//                 "{}_{}/{}.bin",
//                 free_cells, throws_left, points_above
//             ))
//             .arg(format!("-otmp/{}/scores/", N))
//             .output()
//             .unwrap();

//         let &cell_ind =
//             CELLS[n_to_ind::<N>()].1[free_cells].get(cells).unwrap();
//         let ind = get_index::<N>(dice, cell_ind);

//         let mut f = File::open(format!(
//             "./tmp/{}/scores/{}_{}/{}.bin",
//             N, free_cells, throws_left, points_above
//         ))
//         .unwrap();

//         f.seek(SeekFrom::Start(ind as u64 * 4)).unwrap();
//         let mut bytes = [0; 4];
//         f.read(&mut bytes).unwrap();

//         f32::from_le_bytes(bytes)
//     };

//     remove_file(format!(
//         "./tmp/{}/scores/{}_{}/{}.bin",
//         N, free_cells, throws_left, points_above
//     ))
//     .unwrap();

//     ans
// }

pub fn start<const N: usize>() {
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

    let mut last_dice = DiceThrow::throw(N);
    let mut throws_left = 2;

    println!("Starting throw:\n{}", last_dice);

    loop {
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
            ["set", "points", cell, pts] => {
                let index = get_yatzy_index::<N>(cell);
                let pts = pts.parse().unwrap();
                points[index] = Some(pts);
                display_points::<N>(&points, None, None);
            }
            ["clear", "points", cell] => {
                let index = get_yatzy_index::<N>(cell);

                points[index] = None;
            }
            ["throw", "dice", n] => {
                let n = n.parse().unwrap();

                let throw = DiceThrow::throw(n);

                println!("{}", throw);

                last_dice = throw;
            }
            ["auto"] => {
                let free_cells: Vec<_> =
                    points.iter().map(|x| x.is_none()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                if throws_left == 0 {
                    let ind = get_cell_strat::<N>(
                        &free_cells,
                        &last_dice,
                        points_above,
                    );

                    let score = last_dice.cell_score::<N>(ind);

                    println!(
                        "Putting {} points in {}.",
                        score,
                        get_index_name::<N>(ind)
                    );

                    points[ind] = Some(score);
                    display_points::<N>(&points, None, None);

                    last_dice = DiceThrow::throw(N);
                    throws_left = 2;

                    println!("New throw:\n{}", last_dice);
                } else {
                    let reroll = get_rethrow_strat::<N>(
                        &free_cells,
                        &last_dice,
                        throws_left,
                        points_above,
                    );

                    println!("Rethrowing:\n{}", reroll);

                    let rethrow =
                        DiceThrow::throw(reroll.count_ones() as usize);

                    last_dice = new_throw(&last_dice, reroll, rethrow);

                    println!("To give:\n{}", last_dice);
                    throws_left -= 1;
                }
            }
            ["advise", dice_left, dice] => {
                let throws_left: usize = dice_left.parse().unwrap();
                if dice.len() != N {
                    continue;
                }
                let mut throw = DiceThrow::from([0u8; 6]);
                for c in dice.chars() {
                    let i = (c as u8 - b'0') as usize;
                    throw[i] += 1;
                }

                println!("You entered:\n{}\n", throw);

                let free_cells: Vec<_> =
                    points.iter().map(|x| x.is_none()).collect();

                let points_above =
                    points.iter().take(6).filter_map(|x| x.as_ref()).sum();

                match throws_left {
                    0 => {
                        let ind = get_cell_strat::<N>(
                            &free_cells,
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
                        let rethrow = get_rethrow_strat::<N>(
                            &free_cells,
                            &throw,
                            throws_left,
                            points_above,
                        );

                        println!("Rethrow:\n{}", rethrow);
                    }
                    _ => unreachable!(),
                }
            }
            // ["expected-remaining"] => {
            //     let free_cells: Vec<_> =
            //         points.iter().map(|x| x.is_none()).collect();
            //     let points_above =
            //         points.iter().take(6).filter_map(|x| x.as_ref()).sum();
            //     let rem_score = get_score::<N>(
            //         &free_cells,
            //         &last_dice,
            //         points_above,
            //         throws_left,
            //     );

            //     println!("expected remaining score is {}", rem_score);
            // }
            // ["expected-total"] => {
            //     let free_cells: Vec<_> =
            //         points.iter().map(|x| x.is_none()).collect();
            //     let points_above =
            //         points.iter().take(6).filter_map(|x| x.as_ref()).sum();
            //     let rem_score = get_score::<N>(
            //         &free_cells,
            //         &last_dice,
            //         points_above,
            //         throws_left,
            //     );

            //     let tot_score =
            //         get_total_score::<N>(&points) as f32 + rem_score;

            //     println!("expected total score is {}", tot_score);
            // }
            ["reset"] => {
                points = vec![
                    None;
                    match N {
                        5 => 15,
                        6 => 20,
                        _ => unreachable!(),
                    }
                ];
                last_dice = DiceThrow::throw(N);
                throws_left = 2;

                println!("Starting throw:\n{}", last_dice);
            }
            _ => println!("Invalid command! {:?}", command),
        }
    }
}
