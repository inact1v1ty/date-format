pub(crate) fn add_leading_zeroes(num: i32, target_length: usize) -> String {
    let base = num.abs().to_string();
    let mut result = String::with_capacity(
        (target_length + if num < 0 { 1 } else { 0 })
            .try_into()
            .unwrap(),
    );

    if num < 0 {
        result.push('-');
    }

    let padding = target_length.checked_sub(base.len()).unwrap_or(0usize);

    for _ in 0..padding {
        result.push('0');
    }

    result.push_str(&base);

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::add_leading_zeroes;

    #[test]
    fn adds_leading_zeros_when_number_has_fewer_digits_than_target_length() {
        assert_eq!(add_leading_zeroes(7, 3), "007");
        assert_eq!(add_leading_zeroes(7, 2), "07");
        assert_eq!(add_leading_zeroes(7, 1), "7");
        assert_eq!(add_leading_zeroes(7, 0), "7");
        assert_eq!(add_leading_zeroes(-7, 3), "-007");
    }
}
