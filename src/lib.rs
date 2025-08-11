pub struct A<const B: usize> {
    c: [i64; B],
    d: i64,
    e: usize,
    f: usize,
}

impl<const B: usize> A<B> {
    pub const fn a() -> Self {
        Self {
            c: [0; B],
            d: 0,
            e: 0,
            f: 0,
        }
    }

    pub fn b(&mut self, g: i64) -> f64 {
        let h = self.c[self.f];
        self.c[self.f] = g;
        self.f = (self.f + 1) % B;
        if self.e < B {
            self.e += 1;
            self.d += g;
        } else {
            self.d += g - h;
        }
        self.d as f64 / self.e as f64
    }
}

#[cfg(test)]
mod z {
    use super::*;

    #[test]
    fn a() {
        let mut b = A::<4>::a();
        let c = b.b(10);
        assert_eq!(c, 10.0);
    }

    #[test]
    fn b() {
        let mut c = A::<3>::a();
        assert_eq!(c.b(5), 5.0);
        assert_eq!(c.b(7), 6.0);
    }

    #[test]
    fn c() {
        let mut d = A::<3>::a();
        assert_eq!(d.b(1), 1.0);
        assert_eq!(d.b(2), 1.5);
        assert_eq!(d.b(3), 2.0);
    }

    #[test]
    fn d() {
        let mut e = A::<3>::a();
        e.b(1);
        e.b(2);
        e.b(3);
        let f = e.b(4);
        assert_eq!(f, 3.0);
        let g = e.b(5);
        assert_eq!(g, 4.0);
    }

    #[test]
    fn e() {
        let mut f = A::<2>::a();
        assert_eq!(f.b(-2), -2.0);
        assert_eq!(f.b(2), 0.0);
        assert_eq!(f.b(4), 3.0);
    }
}
