use crate::error::AppError;

const WEIGHTS: [u32; 10] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

pub struct PeselInfo {
    pub date_of_birth: String,
    pub gender: String,
}

pub fn validate(pesel: &str) -> Result<PeselInfo, AppError> {
    if pesel.len() != 11 {
        return Err(AppError::Validation("PESEL musi mieć 11 cyfr".into()));
    }

    let digits: Vec<u32> = pesel
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| AppError::Validation("PESEL może zawierać tylko cyfry".into())))
        .collect::<Result<Vec<_>, _>>()?;

    // Checksum
    let sum: u32 = digits.iter().zip(WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    let check = (10 - (sum % 10)) % 10;
    if check != digits[10] {
        return Err(AppError::Validation("Nieprawidłowa suma kontrolna PESEL".into()));
    }

    // Extract date of birth
    let year_part = digits[0] * 10 + digits[1];
    let month_part = digits[2] * 10 + digits[3];
    let day = digits[4] * 10 + digits[5];

    let (year, month) = if month_part >= 1 && month_part <= 12 {
        (1900 + year_part, month_part)
    } else if month_part >= 21 && month_part <= 32 {
        (2000 + year_part, month_part - 20)
    } else if month_part >= 41 && month_part <= 52 {
        (2100 + year_part, month_part - 40)
    } else if month_part >= 61 && month_part <= 72 {
        (2200 + year_part, month_part - 60)
    } else if month_part >= 81 && month_part <= 92 {
        (1800 + year_part, month_part - 80)
    } else {
        return Err(AppError::Validation("Nieprawidłowy miesiąc w PESEL".into()));
    };

    let date_of_birth = format!("{:04}-{:02}-{:02}", year, month, day);

    // Gender: 10th digit (index 9) - odd = M, even = K
    let gender = if digits[9] % 2 == 1 { "M" } else { "K" };

    Ok(PeselInfo {
        date_of_birth,
        gender: gender.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pesel_female_1985() {
        let info = validate("85032212342").unwrap();
        assert_eq!(info.date_of_birth, "1985-03-22");
        assert_eq!(info.gender, "K");
    }

    #[test]
    fn test_valid_pesel_female_1992() {
        let info = validate("92061578905").unwrap();
        assert_eq!(info.date_of_birth, "1992-06-15");
        assert_eq!(info.gender, "K");
    }

    #[test]
    fn test_invalid_checksum() {
        assert!(validate("85032212349").is_err());
    }

    #[test]
    fn test_wrong_length() {
        assert!(validate("1234567890").is_err());
    }
}
