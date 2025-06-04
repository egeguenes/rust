fn main() {
    for num in 2..2000 {
        fermat_test(num);
    }
}

fn power_with_mod(base: i32, exp: i32, mod_n: i32) -> i32 {
    let mut temp = exp;
    let mut result = base % mod_n;
    while temp > 1 {
        result = (result * base) % mod_n;
        temp -= 1;
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
    let mut temp = to_test;
    let mut factors = vec!(1);
    for num in 2..to_test {
        if temp%num == 0 {
            while temp%num == 0 {
                temp /= num;
            }
            factors.push(num);
        }
    }
    let mut times_passed = 0;
    let mut times_failed = 0;
    for num in 2..to_test {
        if factors.contains(&num) {
            continue;
        }
        if !fermat_little_theorem(num, to_test) {
            times_failed += 1;
            //println!("not passed for value {}", num);
        } else {
            times_passed += 1;
            //println!("passed for value {}", num);
        }
    }
    if times_failed == 0 {
        println!("{} is carmichael number", to_test);
    }
    //println!("passed for {} times", times_passed);
    //println!("failed for {} times", times_failed);
}
