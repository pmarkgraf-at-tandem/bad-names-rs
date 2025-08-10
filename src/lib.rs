pub fn func(mut a: [f64; X], b: &[f64], c: &[f64]) -> (Vec<f64>, [f64; X]) {
    let d = b.len();
    let e = c.len();
    a[e - 1..e - 1 + d].copy_from_slice(b);
    let mut f = vec![0.0; d];
    for n in 0..d {
        let mut g = 0.0;
        for k in 0..e {
            g += c[k] * a[e - 1 + n - k];
        }
        f[n] = g;
    }
    let mut h = [0.0; X];
    h[..e - 1].copy_from_slice(&a[d..d + e - 1]);
    h[e - 1..e - 1 + d].copy_from_slice(b);
    (f, h)
}

const Z: usize = 80;
const Y: usize = 63;
const X: usize = Y - 1 + Z;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let m = zeros();
        let n = {
            let mut o = vec![0.0; 8];
            o[0] = 1.0;
            o
        };
        let p = [1.0, 0.0, 0.0];
        let (q, r) = func(m, &n, &p);
        assert_eq!(q[0], 1.0);
        for &v in &q[1..] {
            assert_eq!(v, 0.0);
        }
        assert_eq!(r[p.len() - 1], 1.0);
    }

    #[test]
    fn test_two() {
        let m = zeros();
        let n = vec![1.0; 8];
        let o = [0.5, 0.5];
        let (p, _) = func(m, &n, &o);
        assert!((p[0] - 0.5).abs() < 1e-10);
        for &v in &p[1..] {
            assert!((v - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_three() {
        let m = zeros();
        let n = vec![0.0; 8];
        let o = [0.2, 0.3, 0.5];
        let (p, q) = func(m, &n, &o);
        for &v in &p {
            assert_eq!(v, 0.0);
        }
        for &v in &q {
            assert!(v.abs() < 1e-10);
        }
    }

    #[test]
    fn test_four() {
        let m = zeros();
        let n = vec![1.0, 2.0, 3.0, 4.0];
        let o = [1.0, 2.0];
        let (p, _) = func(m, &n, &o);
        let q = [1.0, 4.0, 7.0, 10.0];
        for (o, e) in p.iter().zip(q.iter()) {
            assert!((o - e).abs() < 1e-10);
        }
    }

    fn zeros() -> [f64; X] {
        [0.0; X]
    }
}
