/// A no_std moving average calculator for i64 values over a fixed window size N.
pub struct MovingAverage<const N: usize> {
    buffer: [i64; N],
    sum: i64,
    count: usize,
    index: usize,
}

impl<const N: usize> MovingAverage<N> {
    /// Creates a new MovingAverage with all values initialized to 0.
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            sum: 0,
            count: 0,
            index: 0,
        }
    }

    /// Adds a new value and returns the current moving average as f64.
    pub fn add(&mut self, value: i64) -> f64 {
        let old = self.buffer[self.index];
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % N;
        if self.count < N {
            self.count += 1;
            self.sum += value;
        } else {
            self.sum += value - old;
        }
        self.sum as f64 / self.count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_add() {
        let mut ma = MovingAverage::<4>::new();
        let avg = ma.add(10);
        assert_eq!(avg, 10.0);
    }

    #[test]
    fn test_less_than_n_adds() {
        let mut ma = MovingAverage::<3>::new();
        assert_eq!(ma.add(5), 5.0);
        assert_eq!(ma.add(7), 6.0);
    }

    #[test]
    fn test_exactly_n_adds() {
        let mut ma = MovingAverage::<3>::new();
        assert_eq!(ma.add(1), 1.0);
        assert_eq!(ma.add(2), 1.5);
        assert_eq!(ma.add(3), 2.0);
    }

    #[test]
    fn test_more_than_n_adds() {
        let mut ma = MovingAverage::<3>::new();
        ma.add(1);
        ma.add(2);
        ma.add(3);
        let avg = ma.add(4); // window is now [2,3,4]
        assert_eq!(avg, 3.0);
        let avg2 = ma.add(5); // window is now [3,4,5]
        assert_eq!(avg2, 4.0);
    }

    #[test]
    fn test_negative_values() {
        let mut ma = MovingAverage::<2>::new();
        assert_eq!(ma.add(-2), -2.0);
        assert_eq!(ma.add(2), 0.0);
        assert_eq!(ma.add(4), 3.0);
    }
}
