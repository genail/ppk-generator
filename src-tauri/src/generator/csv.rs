use crate::models::contribution::ContributionWithMember;

/// Build CSV matching exact PZU format:
/// - CRLF line endings
/// - Semicolon delimiter
/// - Header row unquoted
/// - Data rows all-quoted
/// - Comma decimals ("94,38")
/// - Month unpadded
pub fn build(contributions: &[ContributionWithMember], period_year: i32, period_month: i32) -> String {
    let mut csv = String::new();

    // Header (unquoted)
    csv.push_str("LP;NR_PESEL;DOK_TOZSAMOSCI_RODZAJ;DOK_TOZSAMOSCI_SERIA_NUMER;UCZESTNIK_IDENTYFIKATOR_INFORMATYCZNY;NAZWISKO;IMIE;WARTOSC_PODST_PRACOWNIKA;WARTOSC_DODATK_PRACOWNIKA;WARTOSC_PODST_PRACODAWCY;WARTOSC_DODATK_PRACODAWCY;FLAGA_OBNIZENIE_SKL_PODST_PRACOWNIKA;ZA_MIESIAC;ZA_ROK;PZIF_RACH_PPK;ID_EPPK_UCZESTNIKA\r\n");

    for (i, c) in contributions.iter().enumerate() {
        let lp = i + 1;
        csv.push_str(&format!(
            "\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\"",
            lp,
            c.pesel,
            c.doc_type,
            c.doc_number,
            "", // UCZESTNIK_IDENTYFIKATOR_INFORMATYCZNY - empty
            c.last_name.to_uppercase(),
            c.first_name.to_uppercase(),
            format_comma_decimal(&c.employee_basic),
            format_comma_decimal(&c.employee_additional),
            format_comma_decimal(&c.employer_basic),
            format_comma_decimal(&c.employer_additional),
            c.reduced_basic_flag,
            period_month, // unpadded
            period_year,
            "", // PZIF_RACH_PPK - empty
            "", // ID_EPPK_UCZESTNIKA - empty
        ));
        csv.push_str("\r\n");
    }

    csv
}

/// Convert dot decimal to comma decimal ("94.38" -> "94,38")
fn format_comma_decimal(value: &str) -> String {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    match Decimal::from_str(value) {
        Ok(d) => format!("{:.2}", d).replace('.', ","),
        Err(_) => value.replace('.', ","),
    }
}
