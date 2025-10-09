use crate::gate::{and, not, or, xor};

pub fn mux(os: bool, d: bool, x: bool) -> bool {
    let and_left = and(d, not(os));
    let and_right = and(x, os);
    let or = or(and_left, and_right);

    return or;
}

pub fn divider(input_x: Vec<bool>, input_y: Vec<bool>) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    let mut os = true;
    let mut last_row_output: Vec<bool> = vec![];

    for x in (0..input_x.len()).rev() {
        let mut carry = false;
        let mut fs_results: Vec<bool> = vec![];
        let mut cur_row_output: Vec<bool> = vec![];

        for y in 0..input_y.len() {
            if x == input_x.len() - 1 {
                if y == 0 {
                    let fs00 = fs(input_x[x], input_y[y], carry);
                    carry = fs00.carry;
                    fs_results.push(fs00.val);
                } else {
                    let fs = fs(false, input_y[y], carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                }
            } else {
                if y == 0 {
                    let fs = fs(input_x[x], input_y[y], carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                } else {
                    let fs = fs(last_row_output[y - 1], input_y[y], carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                }
            }
        }

        if x == input_x.len() - 1 {
            let fs = fs(false, false, carry);
            os = fs.carry;
            result.push(not(fs.carry));
        } else {
            let fs = fs(last_row_output[input_y.len() - 1], false, carry);
            os = fs.carry;
            result.push(not(fs.carry));
        }

        for y in (0..input_y.len()).rev() {
            if x == input_x.len() - 1 {
                if y == 0 {
                    let mux = mux(os, fs_results[y], input_x[x]);
                    cur_row_output.push(mux);
                } else {
                    let mux = mux(os, fs_results[y], false);
                    cur_row_output.push(mux);
                }
            } else {
                if y == 0 {
                    let mux = mux(os, fs_results[y], input_x[x]);
                    cur_row_output.push(mux);
                } else {
                    let mux = mux(os, fs_results[y], last_row_output[y - 1]);
                    cur_row_output.push(mux);
                }
            }
        }

        cur_row_output.reverse();
        last_row_output = cur_row_output.clone();
    }
    return result;
}

#[derive(PartialEq, Debug)]
pub struct FsResult {
    pub val: bool,
    pub carry: bool,
}

//1-bit full subtractor
pub fn fs(a: bool, b: bool, carry_in: bool) -> FsResult {
    let xor1 = xor(a, b);
    let and1 = and(b, not(a));
    let xor2 = xor(carry_in, xor1);
    let and2 = and(carry_in, not(xor1));
    let carry_out = or(and1, and2);

    return FsResult {
        val: xor2,
        carry: carry_out,
    };
}

#[cfg(test)]
mod test {
    use crate::divisor::{FsResult, fs};

    #[test]
    pub fn test_fs() {
        let fs1 = fs(false, false, false);
        let fs2 = fs(false, false, true);
        let fs3 = fs(false, true, false);
        let fs4 = fs(false, true, true);
        let fs5 = fs(true, false, false);
        let fs6 = fs(true, false, true);
        let fs7 = fs(true, true, false);
        let fs8 = fs(true, true, true);

        assert_eq!(
            fs1,
            FsResult {
                carry: false,
                val: false
            }
        );
        assert_eq!(
            fs2,
            FsResult {
                carry: true,
                val: true
            }
        );
        assert_eq!(
            fs3,
            FsResult {
                carry: true,
                val: true
            }
        );
        assert_eq!(
            fs4,
            FsResult {
                carry: true,
                val: false
            }
        );
        assert_eq!(
            fs5,
            FsResult {
                carry: false,
                val: true
            }
        );
        assert_eq!(
            fs6,
            FsResult {
                carry: false,
                val: false
            }
        );
        assert_eq!(
            fs7,
            FsResult {
                carry: false,
                val: false
            }
        );
        assert_eq!(
            fs8,
            FsResult {
                carry: true,
                val: true
            }
        );
    }
}
