pub struct MontgomeryReduction<T> {
    n: T,
    ninv: T,
    r2: T,
}

macro_rules! impls {
    ($uty:ident, $ity:ident, $uwide:ident) => {
        impl MontgomeryReduction<$uty> {
            #[inline]
            pub fn new(n: $uty) -> Self {
                let ninv = Self::inv(n);
                let rmodn = (n.wrapping_neg() % n) as $uwide;
                let r2 = ((rmodn * rmodn) % n as $uwide) as $uty;
                Self { n, ninv, r2 }
            }

            /// <https://cp-algorithms.com/algebra/montgomery_multiplication.html#fast-inverse-trick>
            #[inline]
            fn inv(n: $uty) -> $uty {
                let mut ninv: $uty = 1;
                for _ in 0..$uty::BITS.ilog2() {
                    ninv = ninv.wrapping_mul((2 as $uty).wrapping_sub(n.wrapping_mul(ninv)));
                }
                debug_assert_eq!(ninv.wrapping_mul(n), 1);
                ninv
            }

            #[inline]
            pub fn redc(&self, x: $uwide) -> $uty {
                debug_assert!(x < (self.n as $uwide) << $uty::BITS);
                let m = (x as $uty).wrapping_mul(self.ninv);
                let t = ((x as $uwide).wrapping_sub(m as $uwide * self.n as $uwide) >> $uty::BITS)
                    as $uty;
                if (t as $ity) < 0 {
                    (t as $ity + self.n as $ity) as $uty
                } else {
                    t
                }
            }

            #[inline]
            pub fn mul_r(&self, x: $uty) -> $uty {
                self.redc(x as $uwide * self.r2 as $uwide)
            }

            #[inline]
            pub fn mul(&self, x: $uty, y: $uty) -> $uty {
                // self.redc(self.redc(self.mul_r(x) as $uwide * self.mul_r(y) as $uwide) as $uwide)
                self.redc(self.mul_r(x) as $uwide * y as $uwide)
            }
        }
    };
}

impls!(u32, i32, u64);
impls!(u64, i64, u128);
