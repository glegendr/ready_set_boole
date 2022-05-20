pub fn adder(a: u32, b: u32) -> u32 {

    let mut ret = 0;
    let mut mem = 0;
    for i in 0..32 {
        let bit_a = (a & (1 << i)) >> i;
        let bit_b = (b & (1 << i)) >> i;

        if bit_a & bit_b == 1 {
            ret = ret | (mem << i);
            mem = 1;
        } else if bit_a ^ bit_b == 1 {
            ret = ret | ((!mem & 1) << i);
        } else {
            ret = ret | (mem << i);
            mem = 0;
        }
    }
    ret
}

/*
 *  1
 * 00
 * 01
 * 00
 * 
 * 00000001
 * 11111110
 * 
 * 01001010 -> 74
 * 00110111 -> 55
 * 10000001 -> 129
 * 
 * 
*/