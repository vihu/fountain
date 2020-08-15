use num_traits::Zero;
use std::ops;

/// Crate-local iterator extensions.
pub(crate) trait IterExt: Iterator {
    fn cumsum<S>(self) -> CumSum<Self, S>
    where
        S: Zero,
        Self: Sized,
    {
        CumSum {
            iter: self,
            sum: S::zero(),
        }
    }
}

impl<I> IterExt for I where I: Iterator {}

pub(crate) struct CumSum<I, S> {
    iter: I,
    sum: S,
}

impl<I, S> Iterator for CumSum<I, S>
where
    I: Iterator,
    S: ops::AddAssign<I::Item> + Copy,
{
    type Item = S;
    fn next(&mut self) -> Option<S> {
        self.iter.next().map(|elem| {
            self.sum += elem;
            self.sum
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IterExt;

    #[test]
    fn cumsum_test_i32() {
        let input = [1_i32, 1, 1, 1, 1, 1];
        let csum: Vec<i32> = input.iter().cumsum().collect();
        let expected = [1_i32, 2, 3, 4, 5, 6];
        assert_eq!(&csum, &expected);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn cumsum_test_f32() {
        let input = [1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0];
        let csum: Vec<f32> = input.iter().cumsum().collect();
        let expected = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(&csum, &expected);
    }
}
