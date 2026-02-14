use crate::error::AppError;

const WEIGHTS: [u32; 8] = [8, 9, 2, 3, 4, 5, 6, 7];

pub fn validate(regon: &str) -> Result<(), AppError> {
    let regon_clean: String = regon.chars().filter(|c| c.is_ascii_digit()).collect();

    if regon_clean.len() != 9 {
        return Err(AppError::Validation("REGON musi mieć 9 cyfr".into()));
    }

    let digits: Vec<u32> = regon_clean
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let sum: u32 = digits[..8].iter().zip(WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    let check = sum % 11;
    let check = if check == 10 { 0 } else { check };

    if check != digits[8] {
        return Err(AppError::Validation("Nieprawidłowa suma kontrolna REGON".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_regon() {
        assert!(validate("123456785").is_ok());
    }

    #[test]
    fn test_invalid_regon() {
        assert!(validate("123456789").is_err());
    }
}
