

pub fn fpow(mut a: u64, mut x: u64, m: u64) -> u64 {
    let mut res: u64 = 1;
    while x > 0 {
        if x & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        x >>= 1;
    }
    res
}