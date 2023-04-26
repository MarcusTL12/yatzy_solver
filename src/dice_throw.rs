use std::{
    fmt::{Display, Error, Formatter},
    ops::{Index, IndexMut},
};

use rand::prelude::*;

use crate::dice_distributions::DICE_ORDER_MAP;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiceThrow {
    dice: [usize; 6],
}

impl Index<usize> for DiceThrow {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.dice[index - 1]
    }
}

impl IndexMut<usize> for DiceThrow {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dice[index - 1]
    }
}

impl Display for DiceThrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        const EYES: [[char; 9]; 6] = [
            [' ', ' ', ' ', ' ', '●', ' ', ' ', ' ', ' '],
            ['●', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '●'],
            ['●', ' ', ' ', ' ', '●', ' ', ' ', ' ', '●'],
            ['●', ' ', '●', ' ', ' ', ' ', '●', ' ', '●'],
            ['●', ' ', '●', ' ', '●', ' ', '●', ' ', '●'],
            ['●', ' ', '●', '●', ' ', '●', '●', ' ', '●'],
        ];

        for i in 1..=6 {
            for _ in 0..self[i] {
                write!(f, "┏━━━━━━━┓")?;
            }
        }
        writeln!(f)?;

        for i in (0..9).step_by(3) {
            for j in 1..=6 {
                let eyes = &EYES[j - 1];
                for _ in 0..self[j] {
                    write!(
                        f,
                        "┃ {} {} {} ┃",
                        eyes[i],
                        eyes[i + 1],
                        eyes[i + 2]
                    )?;
                }
            }
            writeln!(f)?;
        }

        for i in 1..=6 {
            for _ in 0..self[i] {
                write!(f, "┗━━━━━━━┛")?;
            }
        }

        Ok(())
    }
}

impl From<[usize; 6]> for DiceThrow {
    fn from(dice: [usize; 6]) -> Self {
        Self { dice }
    }
}

impl<const N: usize> From<[u8; N]> for DiceThrow {
    fn from(dice: [u8; N]) -> Self {
        let mut d = Self::new();

        for x in dice {
            d[x as usize] += 1;
        }

        d
    }
}

impl DiceThrow {
    fn new() -> Self {
        Self { dice: [0; 6] }
    }

    pub fn throw(n: usize) -> Self {
        let mut dice_throw = Self::new();

        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let eyes = rng.gen_range(1..=6);

            dice_throw[eyes] += 1;
        }

        dice_throw
    }

    pub fn ammount_of<const N: usize>(&self) -> usize {
        self[N] * N
    }

    pub fn pairs<const N: usize>(&self) -> usize {
        let (score, amt) = (1..=6)
            .rev()
            .filter_map(|i| if self[i] >= 2 { Some(i * 2) } else { None })
            .take(N)
            .fold((0, 0), |(a, amt), x| (a + x, amt + 1));

        if amt == N {
            score
        } else {
            0
        }
    }

    pub fn n_of_a_kind<const N: usize>(&self) -> usize {
        (1..=6)
            .rev()
            .find_map(|i| if self[i] >= N { Some(i * N) } else { None })
            .unwrap_or(0)
    }

    pub fn straight<const A: usize, const B: usize>(&self) -> usize {
        if (A..=B).all(|i| self[i] >= 1) {
            (A..=B).sum()
        } else {
            0
        }
    }

    pub fn building<const A: usize, const B: usize>(&self) -> usize {
        if let Some(a) = (1..=6).rev().find(|&i| self[i] >= A) {
            if let Some(b) =
                (1..=6).rev().filter(|&i| i != a).find(|&i| self[i] >= B)
            {
                A * a + B * b
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn chance(&self) -> usize {
        (1..=6).map(|i| self[i] * i).sum()
    }

    pub fn yatzy(&self) -> usize {
        let amt_dice: usize = (1..=6).map(|i| self[i]).sum();

        if (1..=6).any(|i| self[i] == amt_dice) {
            match amt_dice {
                5 => 50,
                6 => 100,
                _ => unreachable!(),
            }
        } else {
            0
        }
    }

    pub fn cell_score<const N: usize>(&self, cell_ind: usize) -> usize {
        match (N, cell_ind) {
            (_, 0) => self.ammount_of::<1>(),
            (_, 1) => self.ammount_of::<2>(),
            (_, 2) => self.ammount_of::<3>(),
            (_, 3) => self.ammount_of::<4>(),
            (_, 4) => self.ammount_of::<5>(),
            (_, 5) => self.ammount_of::<6>(),
            (_, 6) => self.pairs::<1>(),
            (_, 7) => self.pairs::<2>(),
            (6, 8) => self.pairs::<3>(),
            (5, 8) | (6, 9) => self.n_of_a_kind::<3>(),
            (5, 9) | (6, 10) => self.n_of_a_kind::<4>(),
            (6, 11) => self.n_of_a_kind::<5>(),
            (5, 10) | (6, 12) => self.straight::<1, 5>(),
            (5, 11) | (6, 13) => self.straight::<2, 6>(),
            (6, 14) => self.straight::<1, 6>(),
            (5, 12) | (6, 15) => self.building::<3, 2>(),
            (6, 16) => self.building::<3, 3>(),
            (6, 17) => self.building::<4, 2>(),
            (5, 13) | (6, 18) => self.chance(),
            (5, 14) | (6, 19) => self.yatzy(),
            _ => unreachable!(),
        }
    }

    pub fn test_all(&self) {
        println!("1:                {}", self.ammount_of::<1>());
        println!("2:                {}", self.ammount_of::<2>());
        println!("3:                {}", self.ammount_of::<3>());
        println!("4:                {}", self.ammount_of::<4>());
        println!("5:                {}", self.ammount_of::<5>());
        println!("6:                {}", self.ammount_of::<6>());
        println!("--------------------");
        println!("1 pair:           {}", self.pairs::<1>());
        println!("2 pair:           {}", self.pairs::<2>());
        println!("3 pair:           {}", self.pairs::<3>());
        println!("3 of a kind:      {}", self.n_of_a_kind::<3>());
        println!("4 of a kind:      {}", self.n_of_a_kind::<4>());
        println!("5 of a kind:      {}", self.n_of_a_kind::<5>());
        println!("Small straight:   {}", self.straight::<1, 5>());
        println!("Large straight:   {}", self.straight::<2, 6>());
        println!("Full  straight:   {}", self.straight::<1, 6>());
        println!("Hut:              {}", self.building::<3, 2>());
        println!("House:            {}", self.building::<3, 3>());
        println!("Tower:            {}", self.building::<4, 2>());
        println!("Chance:           {}", self.chance());
        println!("yatzy:            {}", self.yatzy());
    }

    pub fn into_sub_throw_iter(self) -> SubThrowIter {
        SubThrowIter::from(self)
    }

    pub fn amt_dice(&self) -> usize {
        (1..=6).map(|i| self[i]).sum()
    }

    pub fn probability(&self) -> f64 {
        let amt_dice = self.amt_dice();

        let tot = 6usize.pow(amt_dice as u32);

        let perms: usize = factorial(amt_dice);

        let dup_perms: usize = (1..=6).map(|i| factorial(self[i])).product();

        let actual_perms = perms / dup_perms;

        (actual_perms as f64) / (tot as f64)
    }

    pub fn into_ordered_dice(&self) -> impl Iterator<Item = u8> + '_ {
        self.dice
            .iter()
            .enumerate()
            .flat_map(|(i, &amt)| (0..amt).map(move |_| (i + 1) as u8))
    }

    pub fn into_ordered_dice_const<const N: usize>(&self) -> [u8; N] {
        let mut dice = [0; N];

        for (x, y) in dice.iter_mut().zip(self.into_ordered_dice()) {
            *x = y;
        }

        dice
    }

    pub fn get_index(&self) -> usize {
        match self.amt_dice() {
            0 => 0,
            1 => {
                let dice = self.into_ordered_dice_const::<1>();
                DICE_ORDER_MAP.1[&dice]
            }
            2 => {
                let dice = self.into_ordered_dice_const::<2>();
                DICE_ORDER_MAP.2[&dice]
            }
            3 => {
                let dice = self.into_ordered_dice_const::<3>();
                DICE_ORDER_MAP.3[&dice]
            }
            4 => {
                let dice = self.into_ordered_dice_const::<4>();
                DICE_ORDER_MAP.4[&dice]
            }
            5 => {
                let dice = self.into_ordered_dice_const::<5>();
                DICE_ORDER_MAP.5[&dice]
            }
            6 => {
                let dice = self.into_ordered_dice_const::<6>();
                DICE_ORDER_MAP.6[&dice]
            }
            _ => panic!("Unsupported amount of dice!"),
        }
    }

    pub fn collect_dice<const N: usize>(&self) -> [u8; N] {
        let mut dice = [0; N];
        for (i, d) in self.into_ordered_dice().take(N).enumerate() {
            dice[i] = d;
        }
        dice
    }

    pub fn overwrite_reroll<const M: usize, const N: usize>(
        &self,
        mut mask: u8,
        new_dice: [u8; N],
    ) -> Self {
        let mut dice: [u8; M] = self.collect_dice();

        let mut j = 0;
        for die in dice.iter_mut() {
            if mask & 1 == 1 {
                *die = new_dice[j];
                j += 1;
            }
            mask >>= 1;
        }

        Self::from(dice)
    }

    pub fn overwrite_reroll_dyn<const M: usize>(
        &self,
        mut mask: u8,
        new_dice: &[u8],
    ) -> Self {
        let mut dice: [u8; M] = self.collect_dice();

        let mut j = 0;
        for die in dice.iter_mut() {
            if mask & 1 == 1 {
                *die = new_dice[j];
                j += 1;
            }
            mask >>= 1;
        }

        Self::from(dice)
    }

    pub fn get_mask(&self, mut sub_throw: DiceThrow) -> u8 {
        let mut mask = 0;

        let bit = 1 << (self.amt_dice() - 1);

        for d in self.into_ordered_dice() {
            mask >>= 1;
            if sub_throw[d as usize] > 0 {
                mask |= bit;
                sub_throw[d as usize] -= 1;
            }
        }

        mask
    }

    pub fn into_mask_iter(self) -> impl Iterator<Item = u8> {
        self.into_sub_throw_iter().map(move |st| self.get_mask(st))
    }

    pub fn get_subthrow(&self, mut mask: u8) -> Self {
        let mut dice = DiceThrow::new();

        for d in self.into_ordered_dice() {
            if mask & 1 != 0 {
                dice[d as usize] += 1;
            }

            mask >>= 1;
        }

        dice
    }
}

fn factorial(n: usize) -> usize {
    (2..=n).product()
}

pub struct SubThrowIter {
    done: bool,
    dice: DiceThrow,
    sub_throw: DiceThrow,
}

impl From<DiceThrow> for SubThrowIter {
    fn from(dice: DiceThrow) -> Self {
        SubThrowIter {
            done: false,
            dice,
            sub_throw: DiceThrow::new(),
        }
    }
}

impl Iterator for SubThrowIter {
    type Item = DiceThrow;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let ans = Some(self.sub_throw);
            for i in 1..=6 {
                if self.sub_throw[i] < self.dice[i] {
                    self.sub_throw[i] += 1;
                    break;
                } else {
                    self.sub_throw[i] = 0;
                    if i == 6 {
                        self.done = true;
                    }
                }
            }
            ans
        }
    }
}
