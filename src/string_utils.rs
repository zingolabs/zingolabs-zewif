pub fn format_with_underscores(amount: impl Into<u64>) -> String {
    let s = amount.into().to_string();
    let mut result = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('_');
        }
        result.push(c);
    }

    result.chars().rev().collect()
}

pub fn format_zats_as_zec(amount: impl Into<u64>) -> String {
    let amount = amount.into();
    let integer = amount / 100_000_000;
    let fraction = amount % 100_000_000;
    if fraction == 0 {
        return format!("ZEC {}.0", integer);
    }
    // Format fractional part with leading zeros, then remove trailing zeros.
    let fraction_str = format!("{:08}", fraction);
    let trimmed_fraction = fraction_str.trim_end_matches('0');
    format!("ZEC {}.{}", integer, trimmed_fraction)
}

pub fn format_signed_zats_as_zec(amount: impl Into<i64>) -> String {
    let amount = amount.into();
    if amount < 0 {
        format!("-{}", format_zats_as_zec(-amount as u64))
    } else {
        format_zats_as_zec(amount as u64)
    }
}
