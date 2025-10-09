use crate::{
    divisor::{fs, mux},
    gate::not,
};

pub fn square_root(input: Vec<bool>) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    let mut cur_row_csms_count = 3;
    let mut os_inv_outs: Vec<bool> = vec![];
    let mut last_row_csm_outputs: Vec<bool> = vec![];

    for x in (0..input.len()).step_by(2) {
        let mut cur_row_csm_outputs: Vec<bool> = vec![];
        let cur_row_used_bits = vec![input[x], input[x + 1]];
        let mut carry = false;
        let mut fs_results: Vec<bool> = vec![];
        let mut os = false;

        for csm in 0..cur_row_csms_count {
            if x == 0 {
                if csm == 0 {
                    let fs = fs(cur_row_used_bits[1], true, carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                } else if csm == cur_row_csms_count - 1 {
                    let fs = fs(false, false, carry);
                    os = fs.carry;
                    result.push(not(fs.carry));
                } else {
                    let fs = fs(cur_row_used_bits[0], false, carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                }
            } else {
                if csm == 0 {
                    let fs = fs(cur_row_used_bits[1], true, carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                } else if csm == 1 {
                    let fs = fs(cur_row_used_bits[0], false, carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                } else if csm == cur_row_csms_count - 1 {
                    let fs = fs(last_row_csm_outputs[csm - 2], false, carry);
                    os = fs.carry;
                    result.push(not(fs.carry));
                } else {
                    let fs = fs(last_row_csm_outputs[csm - 2], os_inv_outs[csm - 2], carry);
                    carry = fs.carry;
                    fs_results.push(fs.val);
                }
            }
        }

        for csm in (0..cur_row_csms_count - 1).rev() {
            if x == 0 {
                if csm == 0 {
                    let mux = mux(os, fs_results[csm], cur_row_used_bits[1]);
                    cur_row_csm_outputs.push(mux);
                    os_inv_outs.push(not(os));
                } else if csm == 1 {
                    let mux = mux(os, fs_results[csm], cur_row_used_bits[0]);
                    cur_row_csm_outputs.push(mux);
                }
            } else {
                if csm == 0 {
                    let mux = mux(os, fs_results[csm], cur_row_used_bits[1]);
                    os_inv_outs.insert(0, not(os));
                    cur_row_csm_outputs.push(mux);
                } else if csm == 1 {
                    let mux = mux(os, fs_results[csm], cur_row_used_bits[0]);
                    cur_row_csm_outputs.push(mux);
                } else {
                    let mux = mux(os, fs_results[csm], last_row_csm_outputs[csm - 2]);
                    cur_row_csm_outputs.push(mux);
                }
            }
        }

        //os_inv_outs.insert(0, element);
        cur_row_csms_count += 1;
        cur_row_csm_outputs.reverse();
        last_row_csm_outputs = cur_row_csm_outputs.clone();
    }
    return result;
}
