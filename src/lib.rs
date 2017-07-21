extern crate sum;

pub use sum::*;

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 1), 2);
    }
}
