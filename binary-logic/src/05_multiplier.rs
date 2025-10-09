use crate::{
    adder::{binary_adder, full_adder},
    gate::and,
};

pub fn multiplier(input_a: Vec<bool>, input_b: Vec<bool>) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];

    let mut last_level_carry: bool = false;
    let mut level_out: Vec<bool> = vec![];

    for b in 0..input_b.len() {
        let mut cur_level_carry: bool = false;

        for a in 0..input_a.len() {
            let g = and(input_a[a], input_b[b]);

            if b == 0 {
                if a == 0 {
                    result.push(g);
                } else {
                    level_out.push(g);
                }
            } else if b == 1 {
                if a == 0 {
                    let ba = binary_adder(g, level_out[0]);
                    cur_level_carry = ba.carry;
                    result.push(ba.sum);
                } else if a == input_a.len() - 1 {
                    let ba = binary_adder(g, cur_level_carry);
                    level_out[a - 1] = ba.sum;
                    last_level_carry = ba.carry;
                } else {
                    let fa = full_adder(g, level_out[a], cur_level_carry);
                    cur_level_carry = fa.carry;
                    level_out[a - 1] = fa.sum;
                }
            } else {
                if a == 0 {
                    let ba = binary_adder(g, level_out[0]);
                    cur_level_carry = ba.carry;
                    result.push(ba.sum);
                } else if a == input_a.len() - 1 {
                    let fa = full_adder(g, last_level_carry, cur_level_carry);
                    last_level_carry = fa.carry;
                    level_out[a - 1] = fa.sum;
                } else {
                    let fa = full_adder(g, level_out[a], cur_level_carry);
                    cur_level_carry = fa.carry;
                    level_out[a - 1] = fa.sum;
                }
            }
        }

        if b == input_b.len() {
            level_out.reverse();
            result = [result.as_slice(), level_out.as_slice()].concat();
        }
    }

    result.reverse();
    return result;
}
