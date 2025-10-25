pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;
    if n < 2 {
        return factors;
    }

    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    let mut f: u64 = 3;
    while f <= number / f { 
        while number % f == 0 {
            factors.push(f);
            number /= f;
        }
        f += 2;
    }

    if number > 1 {
        factors.push(number);
    }

    factors
}

