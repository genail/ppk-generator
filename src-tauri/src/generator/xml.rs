use crate::models::contribution::ContributionWithMember;
use crate::models::organization::Organization;

/// Build XML matching exact PZU format:
/// - UTF-8, CRLF line endings
/// - 4-space indent
/// - Dot decimals ("94.38")
/// - Empty elements as `<TAG></TAG>`
/// - Period as "YYYY-MM"
pub fn build(
    org: &Organization,
    contributions: &[ContributionWithMember],
    period_year: i32,
    period_month: i32,
) -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let period = format!("{:04}-{:02}", period_year, period_month);

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\r\n");
    xml.push_str("<PPK>\r\n");
    xml.push_str(&format!("    <WERSJA>GRUPA_PPK 1.00</WERSJA>\r\n"));
    xml.push_str(&format!("    <GENERACJA>{}</GENERACJA>\r\n", now));
    xml.push_str("    <PRACODAWCA>\r\n");
    xml.push_str(&format!("        <NIP>{}</NIP>\r\n", org.nip));
    xml.push_str(&format!("        <REGON>{}</REGON>\r\n", org.regon));
    xml.push_str(&format!("        <KONTAKT>{}</KONTAKT>\r\n", org.contact_person));
    xml.push_str("    </PRACODAWCA>\r\n");
    xml.push_str("    <DANE_UCZESTNIKA>\r\n");

    for c in contributions {
        xml.push_str("        <UCZESTNIK>\r\n");
        xml.push_str(&format!("            <NR_PESEL>{}</NR_PESEL>\r\n", c.pesel));
        xml.push_str(&format!("            <DOK_TOZ_TYP>{}</DOK_TOZ_TYP>\r\n", c.doc_type));
        xml.push_str(&format!("            <DOK_TOZ_SYM>{}</DOK_TOZ_SYM>\r\n", c.doc_number));
        xml.push_str(&format!("            <NAZWISKO>{}</NAZWISKO>\r\n", c.last_name.to_uppercase()));
        xml.push_str(&format!("            <IMIE>{}</IMIE>\r\n", c.first_name.to_uppercase()));
        xml.push_str(&format!("            <PLEC>{}</PLEC>\r\n", c.gender));
        xml.push_str(&format!("            <IMIE_2>{}</IMIE_2>\r\n", c.second_name.to_uppercase()));
        xml.push_str(&format!("            <OBYW>{}</OBYW>\r\n", c.citizenship));
        xml.push_str(&format!("            <DATA_UR>{}</DATA_UR>\r\n", c.date_of_birth));
        xml.push_str("            <SKLADKA>\r\n");
        xml.push_str(&format!("                <UCZ_WAR_POD>{}</UCZ_WAR_POD>\r\n", format_dot_decimal(&c.employee_basic)));
        xml.push_str(&format!("                <UCZ_WAR_DOD>{}</UCZ_WAR_DOD>\r\n", format_dot_decimal(&c.employee_additional)));
        xml.push_str(&format!("                <FIR_WAR_POD>{}</FIR_WAR_POD>\r\n", format_dot_decimal(&c.employer_basic)));
        xml.push_str(&format!("                <FIR_WAR_DOD>{}</FIR_WAR_DOD>\r\n", format_dot_decimal(&c.employer_additional)));
        xml.push_str(&format!("                <UCZ_OBNIZ_SKL_POD>{}</UCZ_OBNIZ_SKL_POD>\r\n", c.reduced_basic_flag));
        xml.push_str(&format!("                <SKL_ZA_OKRES>{}</SKL_ZA_OKRES>\r\n", period));
        xml.push_str("            </SKLADKA>\r\n");
        xml.push_str("        </UCZESTNIK>\r\n");
    }

    xml.push_str("    </DANE_UCZESTNIKA>\r\n");
    xml.push_str("</PPK>\r\n");

    xml
}

/// Ensure decimal uses dot and has 2 decimal places
fn format_dot_decimal(value: &str) -> String {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    match Decimal::from_str(value) {
        Ok(d) => format!("{:.2}", d),
        Err(_) => value.to_string(),
    }
}
