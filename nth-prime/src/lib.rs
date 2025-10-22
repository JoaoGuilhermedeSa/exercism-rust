pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;

    while count < n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }

    num
}

fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2..=((x as f64).sqrt() as u32) {
        if x % i == 0 {
            return false;
        }
    }
    true
}