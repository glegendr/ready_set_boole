pub fn gray_code(n: u32) -> u32 {
    let mut ret = 0;
    for i in 0..31 {
        ret |= (((n >> i) & 1) ^ (((n >> i) & 0b10) >> 1)) << i;
    }
    ret |= n & (1 << 31);
    ret
}