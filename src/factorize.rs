use crate::montgomery::Montgomery;

pub fn factorize(mut n: u64) -> Vec<u64> {
    let mut res = vec![];
    while n % 2 == 0 {
        n /= 2;
        res.push(2);
    }
    factorize_rec(n, &mut res);
    res
}

fn factorize_rec(n: u64, res: &mut Vec<u64>) {
    let m = Montgomery::<u64>::new(n);
    if is_prime(&m) {
        res.push(n);
    } else {
        let a = rho(&m);
        let b = n / a;
        factorize_rec(a, res);
        factorize_rec(b, res);
    }
}

fn is_prime(m: &Montgomery<u64>) -> bool {
    if m.n % 2 == 0 {
        return if m.n == 2 { true } else { false };
    }
    let a: &[u64] = if m.n < 4759123141 {
        &[2, 7, 61]
    } else {
        &[2, 325, 9375, 28178, 450775, 9780504, 1795265022]
    };
    a.iter().all(|&a| a >= m.n || miller_rabin(a, m))
}

fn miller_rabin(a: u64, m: &Montgomery<u64>) -> bool {
    let s = (m.n - 1).trailing_zeros();
    let d = (m.n - 1) >> s;
    let mut x = mod_pow(a, d, m);
    if x == 1 || x == m.n - 1 {
        return true;
    }
    let mut xr = m.mul_r(x);
    for _ in 1..s {
        xr = m.redc(xr as u128 * xr as u128);
        x = m.redc(xr as u128);
        if x == m.n - 1 {
            return true;
        }
    }
    false
}

fn mod_pow(n: u64, mut e: u64, m: &Montgomery<u64>) -> u64 {
    if e == 0 {
        return 1;
    }
    let mut base_mul_r = m.mul_r(n);
    let mut acc = 1;
    while e > 0 {
        if e % 2 == 1 {
            acc = m.redc(acc as u128 * base_mul_r as u128);
        }
        e /= 2;
        base_mul_r = m.redc(base_mul_r as u128 * base_mul_r as u128);
    }
    acc
}

fn rho(m: &Montgomery<u64>) -> u64 {
    for i in 1..m.n {
        for j in 2..m.n {
            let mut x = j;
            let mut y = x;
            loop {
                let step = |x| m.redc(x as u128 * x as u128 + i as u128);
                x = step(x);
                y = step(step(y));
                let d = gcd(x.abs_diff(y), m.n);
                if d == m.n {
                    break;
                }
                if d != 1 {
                    return d;
                }
            }
        }
    }
    panic!()
}

fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let tzx = x.trailing_zeros();
    let tzy = y.trailing_zeros();
    let tzg = tzx.min(tzy);
    let mut x = x >> tzx;
    let mut y = y >> tzy;
    while x != y {
        if x < y {
            std::mem::swap(&mut x, &mut y);
        }
        x -= y;
        x >>= x.trailing_zeros();
    }
    x << tzg
}
