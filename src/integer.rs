pub fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let shift = (x | y).trailing_zeros();
    x >>= x.trailing_zeros();
    y >>= y.trailing_zeros();
    while x != y {
        if x > y {
            x -= y;
            x >>= x.trailing_zeros();
        } else {
            y -= x;
            y >>= y.trailing_zeros();
        }
    }
    x << shift
}

pub fn lcm(x: i64, y: i64) -> i64 {
    if x == 0 || y == 0 {
        0
    } else {
        (x / gcd(x, y) * y).abs()
    }
}

pub fn ext_gcd(mut x: i64, mut y: i64) -> (i64, i64, i64) {
    let mut u = [1, 0];
    let mut v = [0, 1];
    loop {
        let q = y / x;
        y = y % x;
        v[0] -= q * u[0];
        v[1] -= q * u[1];
        if y == 0 {
            return if x >= 0 {
                (x, u[0], u[1])
            } else {
                (-x, -u[0], -u[1])
            };
        }

        let q = x / y;
        x = x % y;
        u[0] -= q * v[0];
        u[1] -= q * v[1];
        if x == 0 {
            return if y >= 0 {
                (y, v[0], v[1])
            } else {
                (-y, -v[0], -v[1])
            };
        }
    }
}

pub fn crt(a: i64, m: i64, b: i64, n: i64) -> Option<(i64, i64)> {
    assert!(m > 0);
    assert!(n > 0);

    // mx + a = b (mod n)
    // gx = (b - a)M (mod n) (mM = g (mod n))
    let (a, m, b, n) = if a <= b { (a, m, b, n) } else { (b, n, a, m) };
    let (g, m_inv, _) = ext_gcd(m, n);
    if g == 1 {
        let x = (b - a) * m_inv % n;
        let k = m * n;
        Some(((m * x + a).rem_euclid(k), k))
    } else {
        ((b - a) % g == 0).then(|| {
            let x = (b - a) / g * m_inv % n;
            let k = m / g * n;
            ((m * x + a).rem_euclid(k), k)
        })
    }
}

/*
x = a (mod m)
x = b (mod n)

mM + nN = g
s = anN + bmM
  = a(g - mM) + bmM
s = ag (mod m)
s = bg (mod n)

x = (anN + bmM) / g
  = floor(a/g)nN + floor(b/g)mM + (cnN + dmM) / g
floor(a/g)nN + floor(b/g)mM = floor(a/g)g (mod m)
(cnN + dmM) / g = a - floor(a/g)g = c (mod m)
(cnN + dmM) / g
= (c(g - mM) + dmM) / g
= c(1 - (m/g)M) + d(m/g)M
= c + (d - c)(m/g)M
= c (mod m) (d = c)
*/
