use crate::{
    adder::adder, binary32::Binary32, divisor::divider, multiplier::multiplier, sqrt::square_root,
    subber::subber,
};

macro_rules! import {
    ($file_path:literal, $mod_name:ident) => {
        #[path = $file_path]
        mod $mod_name;
    };
}

import!("lib/binary32.rs", binary32);
import!("01_transistor.rs", transistor);
import!("02_gate.rs", gate);
import!("03_adder.rs", adder);
import!("04_subber.rs", subber);
import!("05_multiplier.rs", multiplier);
import!("06_divisor.rs", divisor);
import!("07_sqrt.rs", sqrt);

fn main() {
    convert_numbers();
}

fn convert_numbers() {
    for n in -10..10 {
        let bin = Binary32::from_dec(n);
        let dec = bin.to_dec();
        println!("init {0} bin {1} dec {2}", n, bin, dec);
    }
}

pub fn add_i32_numbers(input_a: i32, input_b: i32) -> i32 {
    let mut a = Binary32::from_dec(input_a).bits;
    let mut b = Binary32::from_dec(input_b).bits;

    a.reverse();
    b.reverse();

    let result = adder(a, b, false);
    let dec_result = Binary32::new(result).to_dec();
    return dec_result;
}

pub fn sub_i32_numbers(input_a: i32, input_b: i32) -> i32 {
    let mut a = Binary32::from_dec(input_a).bits;
    let mut b = Binary32::from_dec(input_b).bits;

    a.reverse();
    b.reverse();

    let result = subber(a, b);
    let dec_result = Binary32::new(result).to_dec();
    return dec_result;
}

pub fn mul_i32_numbers(input_a: i32, input_b: i32) -> i32 {
    let mut a = Binary32::from_dec(input_a).bits;
    let mut b = Binary32::from_dec(input_b).bits;

    a.reverse();
    b.reverse();

    let result = multiplier(a, b);
    let dec_result = Binary32::new(result).to_dec();
    return dec_result;
}

pub fn div_i32_numbers(input_a: i32, input_b: i32) -> i32 {
    let mut a = Binary32::from_dec(input_a).bits;
    let mut b = Binary32::from_dec(input_b).bits;

    a.reverse();
    b.reverse();

    let result = divider(a, b);
    let dec_result = Binary32::new(result).to_dec();
    return dec_result;
}

pub fn sqrt_i32_number(input: i32) -> i32 {
    let a = Binary32::from_dec(input).bits;
    //a.reverse();
    let result = square_root(a);
    let dec_result = Binary32::new(result).to_dec();
    return dec_result;
}

#[cfg(test)]
mod test {
    use crate::{
        add_i32_numbers, div_i32_numbers, mul_i32_numbers, sqrt_i32_number, sub_i32_numbers,
    };

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt_i32_number(169), 13);
        assert_eq!(sqrt_i32_number(4), 2);
        assert_eq!(sqrt_i32_number(202806081), 14241);
    }

    #[test]
    fn test_div() {
        assert_eq!(div_i32_numbers(130, 13), 10);
        assert_eq!(div_i32_numbers(5, 2), 2);
        assert_eq!(div_i32_numbers(821735651, 2), 410867825);
    }

    #[test]
    fn test_mul_positive() {
        assert_eq!(mul_i32_numbers(1, 1), 1);
        assert_eq!(mul_i32_numbers(2, 2), 4);
        assert_eq!(mul_i32_numbers(1, 10), 10);
        assert_eq!(mul_i32_numbers(14, 1), 14);
        assert_eq!(mul_i32_numbers(100, 100), 10000);
        assert_eq!(mul_i32_numbers(34, 35), 1190);
        assert_eq!(mul_i32_numbers(1234, 21353), 26349602);
    }

    #[test]
    fn test_add_positive() {
        assert_eq!(add_i32_numbers(34, 35), 69);
        assert_eq!(add_i32_numbers(23, 35), 58);
        assert_eq!(add_i32_numbers(0, 35), 35);
        assert_eq!(add_i32_numbers(0, 0), 0);
        assert_eq!(add_i32_numbers(23, 0), 23);
        //im not sure yet where to handle overflows
        assert_eq!(add_i32_numbers(i32::MAX, 1), -2147483648);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub_i32_numbers(50, 32), 18);
        assert_eq!(sub_i32_numbers(-50, 32), -82);
        assert_eq!(sub_i32_numbers(0, 32), -32);
        assert_eq!(sub_i32_numbers(50, 0), 50);
        assert_eq!(sub_i32_numbers(0, 0), 0);
        assert_eq!(sub_i32_numbers(-2147483648, 1), 2147483647);
    }

    fn test_convert_numbers() {}
}
