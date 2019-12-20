use std::f64::INFINITY;
use std::fmt;

use crate::errors::*;
use crate::{Calculate, Low, Next, Reset};

/// Returns the lowest value in a given time frame.
///
/// # Parameters
///
/// * _n_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::Minimum;
/// use ta::{Calculate, Next};
///
/// let mut min = Minimum::new(3).unwrap();
/// assert_eq!(min.calc(10.0), 10.0);
/// assert_eq!(min.calc(11.0), 10.0);
/// assert_eq!(min.calc(12.0), 10.0);
/// assert_eq!(min.calc(13.0), 11.0);
/// ```
#[derive(Debug, Clone)]
pub struct Minimum {
    n: usize,
    vec: Vec<f64>,
    min_index: usize,
    cur_index: usize,
}

impl Minimum {
    pub fn new(n: u32) -> Result<Self> {
        let n = n as usize;

        if n <= 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }

        let indicator = Self {
            n: n,
            vec: vec![INFINITY; n],
            min_index: 0,
            cur_index: 0,
        };

        Ok(indicator)
    }

    fn find_min_index(&self) -> usize {
        let mut min = ::std::f64::INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.vec.iter().enumerate() {
            if val < min {
                min = val;
                index = i;
            }
        }

        index
    }
}

impl Calculate for Minimum {
    fn calc(&mut self, input: f64) -> f64 {
        self.cur_index = (self.cur_index + 1) % (self.n as usize);
        self.vec[self.cur_index] = input;

        if input < self.vec[self.min_index] {
            self.min_index = self.cur_index;
        } else if self.min_index == self.cur_index {
            self.min_index = self.find_min_index();
        }

        self.vec[self.min_index]
    }
}

impl<T: Low> Next<T> for Minimum {
    fn next(&mut self, input: &T) -> f64 {
        self.calc(input.low())
    }
}

impl Reset for Minimum {
    fn reset(&mut self) {
        for i in 0..self.n {
            self.vec[i] = INFINITY;
        }
    }
}

impl Default for Minimum {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for Minimum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(Minimum);

    #[test]
    fn test_new() {
        assert!(Minimum::new(0).is_err());
        assert!(Minimum::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut min = Minimum::new(3).unwrap();

        assert_eq!(min.calc(4.0), 4.0);
        assert_eq!(min.calc(1.2), 1.2);
        assert_eq!(min.calc(5.0), 1.2);
        assert_eq!(min.calc(3.0), 1.2);
        assert_eq!(min.calc(4.0), 3.0);
        assert_eq!(min.calc(6.0), 3.0);
        assert_eq!(min.calc(7.0), 4.0);
        assert_eq!(min.calc(8.0), 6.0);
        assert_eq!(min.calc(-9.0), -9.0);
        assert_eq!(min.calc(0.0), -9.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(low: f64) -> Bar {
            Bar::new().low(low)
        }

        let mut min = Minimum::new(3).unwrap();

        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(1.2)), 1.2);
        assert_eq!(min.next(&bar(5.0)), 1.2);
    }

    #[test]
    fn test_reset() {
        let mut min = Minimum::new(10).unwrap();

        assert_eq!(min.calc(5.0), 5.0);
        assert_eq!(min.calc(7.0), 5.0);

        min.reset();
        assert_eq!(min.calc(8.0), 8.0);
    }

    #[test]
    fn test_default() {
        Minimum::default();
    }

    #[test]
    fn test_display() {
        let indicator = Minimum::new(10).unwrap();
        assert_eq!(format!("{}", indicator), "MIN(10)");
    }
}
