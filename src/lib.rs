//! Luhn checksum. Standard library only.

/// True if the digit string passes the Luhn check.
pub fn is_valid(number: &str) -> bool {
    let mut sum = 0u32;
    let mut alt = false;
    for c in number.chars().rev() {
        if let Some(mut d) = c.to_digit(10) {
            if alt {
                d *= 2;
                if d > 9 { d -= 9; }
            }
            sum += d;
            alt = !alt;
        }
    }
    sum % 10 == 0
}

/// The check digit that makes `partial` a valid Luhn string.
pub fn check_digit(partial: &str) -> u32 {
    (0..10).find(|d| is_valid(&format!("{partial}{d}"))).expect("unreachable")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_vectors() {
        assert!(is_valid("79927398713"));
        assert!(!is_valid("79927398710"));
        assert_eq!(check_digit("7992739871"), 3);
    }
}
