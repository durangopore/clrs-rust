use std::ops::AddAssign;

pub fn sum<'a, T>(values: &'a [T]) -> T
where
    T: AddAssign<T> + Default + Copy,
{
    let mut result = T::default();
    for value in values {
        result += *value;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input: [u8; 0] = [0; 0];
        assert_eq!(sum(&input), 0)
    }

    #[test]
    fn non_empty() {
        let input = [2i128, 3, 5];
        assert_eq!(sum(&input), 10)
    }

    #[derive(Debug, Default, PartialEq, Clone, Copy)]
    struct Complex {
        real: i128,
        imag: i128,
    }

    impl AddAssign for Complex {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    #[test]
    fn custom() {
        let input = [
            Complex { real: 5, imag: 3 },
            Complex { real: -5, imag: 5 },
            Complex { real: 10, imag: -1 },
        ];
        assert_eq!(sum(&input), Complex { real: 10, imag: 7 })
    }
}
