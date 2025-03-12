use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    type Wide: PartialOrd;
    fn wide_mul(self, other: Self) -> Self::Wide;
    fn to_wide(self) -> Self::Wide;
}

macro_rules! int {
    ($ty:ident, $wide:ident) => {
        impl Scalar for $ty {
            type Wide = $wide;
            fn wide_mul(self, other: Self) -> Self::Wide {
                (self as $wide) * (other as $wide)
            }
            fn to_wide(self) -> Self::Wide {
                self as $wide
            }
        }
    };
}
macro_rules! float {
    ($ty:ident) => {
        impl Scalar for $ty {
            type Wide = Self;
            fn wide_mul(self, other: Self) -> Self::Wide {
                self * other
            }
            fn to_wide(self) -> Self::Wide {
                self
            }
        }
    };
}

int!(i32, i64);
int!(i64, i128);
float!(f32);
float!(f64);

pub struct Cht<T> {
    lines: Vec<Line<T>>,
}

impl<T> Cht<T> {
    pub const fn new() -> Self {
        Self { lines: vec![] }
    }
}

impl<T: Scalar> Cht<T> {
    pub fn push_line(&mut self, l: Line<T>) {
        assert!(self.lines.last().is_none_or(|last| last.a >= l.a));
        if let Some(t) = self.lines.last() {
            if t.a == l.a {
                if t.b <= l.b {
                    return;
                }
                self.lines.pop();
            }
        }
        while let [.., s, t] = &*self.lines {
            if (l.b - t.b).wide_mul(s.a - t.a) > (t.b - s.b).wide_mul(t.a - l.a) {
                break;
            }
            self.lines.pop();
        }
        self.lines.push(l);
    }

    pub fn y_min(&self, x: T) -> T {
        assert!(!self.lines.is_empty());
        let mut l = 0;
        let mut r = self.lines.len();
        while r - l > 1 {
            let i = (l + r) / 2;
            let [s, t] = &self.lines[i - 1..i + 1] else {
                unreachable!()
            };
            if x.wide_mul(s.a - t.a) < (t.b - s.b).to_wide() {
                r = i;
            } else {
                l = i;
            }
        }
        self.lines[l].y(x)
    }
}

impl<T: Scalar + Ord> FromIterator<Line<T>> for Cht<T> {
    fn from_iter<I: IntoIterator<Item = Line<T>>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<T: Scalar + Ord> From<Vec<Line<T>>> for Cht<T> {
    fn from(mut lines: Vec<Line<T>>) -> Self {
        use std::cmp::Reverse;
        lines.sort_by_key(|l| Reverse(l.a));
        let mut cht = Self::new();
        for l in lines {
            cht.push_line(l);
        }
        cht
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Line<T> {
    pub a: T,
    pub b: T,
}

impl<T: Scalar> Line<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn y(&self, x: T) -> T {
        self.a * x + self.b
    }
}

pub fn line<T>(a: T, b: T) -> Line<T> {
    Line { a, b }
}
