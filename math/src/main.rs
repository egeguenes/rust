fn main() {
    fermat_test(561);
}

fn power_with_mod(mut base: i32, mut exp: i32, mod_n: i32) -> i32 {
    base %= mod_n;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_n;
        }
        base = (base * base) % mod_n;
        exp /= 2;
    }
    result
}

fn fermat_little_theorem(a: i32, p: i32) -> bool {
    let result = power_with_mod(a, p-1, p);
    if result == 1 {
        return true
    }
    false
}

fn fermat_test(to_test: i32) {
    for num in 2..to_test {
        if !fermat_little_theorem(num, to_test) {
            println!("test not passed for the value {}", num);
        }
    }
    println!("test passed");
}
