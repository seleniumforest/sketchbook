use crate::gate::{and, or, xor};
use std::cmp;

#[derive(PartialEq, Debug)]
pub struct AdderResult {
    pub sum: bool,
    pub carry: bool,
}

pub fn binary_adder(input_a: bool, input_b: bool) -> AdderResult {
    let g1 = xor(input_a, input_b);
    let g2 = and(input_a, input_b);

    return AdderResult { sum: g1, carry: g2 };
}

pub fn full_adder(input_a: bool, input_b: bool, carry: bool) -> AdderResult {
    let ba1 = binary_adder(input_a, input_b);
    let ba2 = binary_adder(carry, ba1.sum);
    let or_gate = or(ba1.carry, ba2.carry);

    return AdderResult {
        sum: ba2.sum,
        carry: or_gate,
    };
}

pub fn adder(a: Vec<bool>, b: Vec<bool>, c: bool) -> Vec<bool> {
    let biggest = cmp::max(a.len(), b.len());

    let mut result: Vec<bool> = vec![];
    let mut carry = c;

    for n in 0..biggest {
        let add = full_adder(a[n], b[n], carry);
        carry = add.carry;
        result.push(add.sum);
    }

    if carry {
        result.push(true);
    }

    result.reverse();
    return result;
}

#[cfg(test)]
mod AdderTest {
    use crate::adder::{AdderResult, binary_adder, full_adder};

    

    #[test]
    fn test_full() {
        let fa1 = full_adder(false, false, false);
        let fa2 = full_adder(false, false, true);
        let fa3 = full_adder(false, true, false);
        let fa4 = full_adder(false, true, true);
        let fa5 = full_adder(true, false, false);
        let fa6 = full_adder(true, false, true);
        let fa7 = full_adder(true, true, false);
        let fa8 = full_adder(true, true, true);

        assert_eq!(
            fa1,
            AdderResult {
                sum: false,
                carry: false
            }
        );
        assert_eq!(
            fa2,
            AdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            fa3,
            AdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            fa4,
            AdderResult {
                sum: false,
                carry: true
            }
        );
        assert_eq!(
            fa5,
            AdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            fa6,
            AdderResult {
                sum: false,
                carry: true
            }
        );
        assert_eq!(
            fa7,
            AdderResult {
                sum: false,
                carry: true
            }
        );
        assert_eq!(
            fa8,
            AdderResult {
                sum: true,
                carry: true
            }
        );
    }

    #[test]
    fn test_binary() {
        let r1 = binary_adder(false, false);
        let r2 = binary_adder(false, true);
        let r3 = binary_adder(true, false);
        let r4 = binary_adder(true, true);

        assert_eq!(
            r1,
            AdderResult {
                sum: false,
                carry: false
            }
        );
        assert_eq!(
            r2,
            AdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            r3,
            AdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            r4,
            AdderResult {
                sum: false,
                carry: true
            }
        );
    }
}
