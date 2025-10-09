use crate::{adder::full_adder, gate::xor};

pub fn subber(input_a: Vec<bool>, input_b: Vec<bool>) -> Vec<bool> {
    let mut b_inv = Vec::new();
    for n in 0..input_b.len() {
        b_inv.push(xor(input_b[n], true));
    }

    let mut carry = true;

    let mut result: Vec<bool> = Vec::new();
    for n in 0..input_a.len() {
        let sub = full_adder(input_a[n], b_inv[n], carry);
        carry = sub.carry;
        result.push(sub.sum);
    }
    result.reverse();

    return result;
}
