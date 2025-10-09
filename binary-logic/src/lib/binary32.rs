use std::{fmt::Display, iter};

use crate::adder::adder;

/*
Signed binary number
for now, 32 bit just for testing
*/

#[derive(Debug)]
pub struct Binary32 {
    pub bits: Vec<bool>,
}

impl Display for Binary32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: String = self
            .bits
            .iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect();

        return f.write_str(&result);
    }
}

impl PartialEq for Binary32 {
    fn eq(&self, other: &Self) -> bool {
        if self.bits.len() != other.bits.len() {
            return false;
        }

        for n in 0..(self.bits.len()) {
            if self.bits[n] != other.bits[n] {
                return false;
            }
        }

        return true;
    }
}

impl Binary32 {
    pub fn new(bits: Vec<bool>) -> Binary32 {
        return Binary32 {
            bits: normalize(bits, false),
        };
    }

    pub fn zero() -> Binary32 {
        return Binary32 {
            bits: normalize(vec![false], false),
        };
    }

    pub fn from_dec(dec: i32) -> Binary32 {
        if dec == 0 {
            return Binary32::new(Vec::from(vec![false]));
        }

        if dec == i32::MIN {
            let mut bits = Binary32::zero().bits;
            bits[0] = true;
            return Binary32::new(bits);
        }

        let is_negative = dec < 0;
        let mut not_negative = if is_negative { dec * -1 } else { dec };
        let mut intermediate: Vec<bool> = Vec::new();

        while not_negative != 0 {
            intermediate.push(not_negative % 2 == 1);
            not_negative /= 2;
        }

        let mut normalized = normalize(intermediate, true);
        if is_negative {
            //reverse bits and add 1 to make it negative.
            for bit in normalized.iter_mut() {
                *bit = !*bit;
            }

            //add 0 and 1 as carry bit.
            let a = adder(
                normalized.iter().rev().cloned().collect(),
                Binary32::zero().bits,
                true,
            );
            return Binary32::new(a);
        }

        return Binary32::new(normalized);
    }

    pub fn to_dec(&self) -> i32 {
        let is_negative = self.bits[0] == true;

        let result = self
            .bits
            .iter()
            .enumerate()
            .fold(i64::from(0), |acc, (i, &bit)| {
                if bit {
                    let num = i64::pow(
                        2,
                        u32::try_from(self.bits.len() - 1).unwrap() - u32::try_from(i).unwrap(),
                    );

                    if i == 0 && is_negative {
                        acc - num
                    } else {
                        acc + num
                    }
                } else {
                    acc
                }
            });

        return i32::try_from(result).unwrap();
    }
}

pub fn normalize(bin: Vec<bool>, reverse: bool) -> Vec<bool> {
    if bin.len() > 32 {
        todo!()
    }

    let mut copy = bin.clone();
    if reverse {
        copy.reverse();
    }
    let add_zeros: Vec<bool> = iter::repeat(false).take(32 - copy.len()).collect();
    let result = [add_zeros.as_slice(), copy.as_slice()].concat();

    return result;
}
