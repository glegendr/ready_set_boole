use crate::exo_00::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {

    let mut ret = 0;
    for i in 0..32 {
        let bit_b = (b & (1 << i)) >> i;
        if bit_b == 1 {
            ret = adder(ret, a << i);
        }
    }
    ret
}

/*
 *           11101
 *            1011
 *           -----
 *           11101 << 0  
 *           11101 << 1
 *              /
 *           11101 << 3 
 * 
*/