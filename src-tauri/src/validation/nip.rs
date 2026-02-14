use crate::error::AppError;

const WEIGHTS: [u32; 9] = [6, 5, 7, 2, 3, 4, 5, 6, 7];

pub fn validate(nip: &str) -> Result<(), AppError> {
    // Strip dashes
    let nip_clean: String = nip.chars().filter(|c| c.is_ascii_digit()).collect();

    if nip_clean.len() != 10 {
        return Err(AppError::Validation("NIP musi mieć 10 cyfr".into()));
    }

    let digits: Vec<u32> = nip_clean
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let sum: u32 = digits[..9].iter().zip(WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    let check = sum % 11;

    if check == 10 || check != digits[9] {
        return Err(AppError::Validation("Nieprawidłowa suma kontrolna NIP".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_nip() {
        assert!(validate("5261040828").is_ok());
    }

    #[test]
    fn test_invalid_nip() {
        assert!(validate("1234567890").is_err());
    }
}
