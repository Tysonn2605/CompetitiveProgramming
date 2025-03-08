fn power_mod(base: i64, exp: i64, modulo: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let mut res = power_mod(base, exp / 2, modulo);
    res = (res * res) % modulo;
    if exp % 2 == 1 {
        res = (res * base) % modulo;
    }
    res
}

fn main() {
    let n = 100;
    let b = 2;
    let modulo: i64 = 1E9 as i64 + 7;
    let res = power_mod(b, n, modulo);
    println!("{}", res);
}
