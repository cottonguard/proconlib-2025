pub fn floor_sum(mut n: i64, m: i64, a: i64, b: i64) -> i64 {
    assert!(n >= 0);
    assert_ne!(m, 0);

    if a == 0 {
        return b / m * n;
    }

    let (mut m, mut a, mut b) = if m > 0 { (m, a, b) } else { (-m, -a, -b) };

    let mut sum = 0;
    while n > 0 {
        /*
        floor_sum(n, m, a, b)
        = sum 0<=i<n floor((ai + b) / m)

        floor_sum(n, m, a - km, b)
        = sum 0<=i<n floor(((a - km)i + b) / m)
        = sum 0<=i<n floor((ai + b) / m - ki)
        = sum 0<=i<n floor((ai + b) / m) - kn(n - 1) / 2

        floor_sum(n, m, a, b - km)
        = sum 0<=i<n floor((ai + b - m) / m)
        = sum 0<=i<n floor((ai + b) / m - k)
        = sum 0<=i<n floor((ai + b) / m) - kn

        floor_sum(n, m, a mod m, b mod m)
        = floor_sum(n, m, a - floor(a / m)m, b - floor(b / m)m)
        = sum 0<=i<n floor((ai + b) / m) - floor(a / m)n(n - 1) / 2 - floor(b / m)n
        = floor_sum(n, m, a, b) - floor(a / m)n(n - 1) / 2 - floor(b / m)n

        floor_sum(n, m, a, b) = floor_sum(n, m, a mod m, b mod m) + floor(a / m)n(n - 1) / 2 + floor(b / m)n
         */

        sum += a / m * n * (n - 1) / 2 + b / m * n;
        a %= m;
        b %= m;
        if a < 0 {
            sum -= n * (n - 1) / 2;
            a += m;
        }
        if b < 0 {
            sum -= n;
            b += m;
        }
        debug_assert!(a >= 0 && b >= 0);

        /*
        floor_sum(n, m, a, b) (1 <= a < m, 0 <= b < m)
        = floor_sum(y, a, m, z)

        y = max{floor((ai + b) / m) | 0 <= i <= n}
        = floor((an + b) / m)
        (a(n - z/a) + b) / m = y
        (an + b - z) / m = y

        z = an + b - my
        = an + b - floor((an + b) / m)m
        = (an + b) mod m
         */

        let t = a * n + b;
        n = t / m;
        b = t % m;
        std::mem::swap(&mut a, &mut m);
    }
    sum
}