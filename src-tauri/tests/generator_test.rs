use app_lib::generator;
use app_lib::models::contribution::ContributionWithMember;
use app_lib::models::organization::Organization;

fn sample_org() -> Organization {
    Organization {
        id: 1,
        name: "Test Org".to_string(),
        nip: "5261040828".to_string(),
        regon: "123456785".to_string(),
        contact_person: "Jan Kowalski".to_string(),
        created_at: "2026-01-09".to_string(),
        updated_at: "2026-01-09".to_string(),
    }
}

fn sample_contributions() -> Vec<ContributionWithMember> {
    vec![
        ContributionWithMember {
            id: 1,
            member_id: 1,
            period_year: 2025,
            period_month: 12,
            employee_basic: "94.38".to_string(),
            employee_additional: "0.00".to_string(),
            employer_basic: "70.78".to_string(),
            employer_additional: "0.00".to_string(),
            reduced_basic_flag: "N".to_string(),
            source: "manual".to_string(),
            updated_at: "2026-01-09".to_string(),
            pesel: "85032212342".to_string(),
            first_name: "MARIA".to_string(),
            last_name: "TESTOWA".to_string(),
            gender: "K".to_string(),
            date_of_birth: "1985-03-22".to_string(),
            citizenship: "PL".to_string(),
            second_name: "".to_string(),
            doc_type: "".to_string(),
            doc_number: "".to_string(),
            member_status: "active".to_string(),
        },
        ContributionWithMember {
            id: 2,
            member_id: 2,
            period_year: 2025,
            period_month: 12,
            employee_basic: "188.92".to_string(),
            employee_additional: "0.00".to_string(),
            employer_basic: "141.69".to_string(),
            employer_additional: "0.00".to_string(),
            reduced_basic_flag: "N".to_string(),
            source: "manual".to_string(),
            updated_at: "2026-01-09".to_string(),
            pesel: "92061578905".to_string(),
            first_name: "EWA".to_string(),
            last_name: "PRZYKLADOWA ".to_string(),
            gender: "K".to_string(),
            date_of_birth: "1992-06-15".to_string(),
            citizenship: "PL".to_string(),
            second_name: "".to_string(),
            doc_type: "D".to_string(),
            doc_number: "ABC123456".to_string(),
            member_status: "active".to_string(),
        },
    ]
}

#[test]
fn test_xml_structure_matches_sample() {
    let org = sample_org();
    let contributions = sample_contributions();
    let xml = generator::xml::build(&org, &contributions, 2025, 12);

    // Verify CRLF line endings
    assert!(xml.contains("\r\n"), "XML must use CRLF line endings");

    // Verify XML declaration
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>\r\n"));

    // Verify structure
    assert!(xml.contains("<PPK>\r\n"));
    assert!(xml.contains("    <WERSJA>GRUPA_PPK 1.00</WERSJA>\r\n"));
    assert!(xml.contains("    <PRACODAWCA>\r\n"));
    assert!(xml.contains("        <NIP>5261040828</NIP>\r\n"));
    assert!(xml.contains("        <REGON>123456785</REGON>\r\n"));
    assert!(xml.contains("        <KONTAKT>Jan Kowalski</KONTAKT>\r\n"));
    assert!(xml.contains("    <DANE_UCZESTNIKA>\r\n"));

    // Verify first participant
    assert!(xml.contains("            <NR_PESEL>85032212342</NR_PESEL>\r\n"));
    assert!(xml.contains("            <DOK_TOZ_TYP></DOK_TOZ_TYP>\r\n"));
    assert!(xml.contains("            <DOK_TOZ_SYM></DOK_TOZ_SYM>\r\n"));
    assert!(xml.contains("            <NAZWISKO>TESTOWA</NAZWISKO>\r\n"));
    assert!(xml.contains("            <IMIE>MARIA</IMIE>\r\n"));
    assert!(xml.contains("            <PLEC>K</PLEC>\r\n"));
    assert!(xml.contains("            <IMIE_2></IMIE_2>\r\n"));
    assert!(xml.contains("            <OBYW>PL</OBYW>\r\n"));
    assert!(xml.contains("            <DATA_UR>1985-03-22</DATA_UR>\r\n"));
    assert!(xml.contains("                <UCZ_WAR_POD>94.38</UCZ_WAR_POD>\r\n"));
    assert!(xml.contains("                <UCZ_WAR_DOD>0.00</UCZ_WAR_DOD>\r\n"));
    assert!(xml.contains("                <FIR_WAR_POD>70.78</FIR_WAR_POD>\r\n"));
    assert!(xml.contains("                <FIR_WAR_DOD>0.00</FIR_WAR_DOD>\r\n"));
    assert!(xml.contains("                <UCZ_OBNIZ_SKL_POD>N</UCZ_OBNIZ_SKL_POD>\r\n"));
    assert!(xml.contains("                <SKL_ZA_OKRES>2025-12</SKL_ZA_OKRES>\r\n"));

    // Verify second participant
    assert!(xml.contains("            <NR_PESEL>92061578905</NR_PESEL>\r\n"));
    assert!(xml.contains("            <DOK_TOZ_TYP>D</DOK_TOZ_TYP>\r\n"));
    assert!(xml.contains("            <DOK_TOZ_SYM>ABC123456</DOK_TOZ_SYM>\r\n"));
    assert!(xml.contains("                <UCZ_WAR_POD>188.92</UCZ_WAR_POD>\r\n"));
    assert!(xml.contains("                <FIR_WAR_POD>141.69</FIR_WAR_POD>\r\n"));

    // Verify closing
    assert!(xml.contains("    </DANE_UCZESTNIKA>\r\n"));
    assert!(xml.contains("</PPK>\r\n"));

    // Verify 4-space indentation
    assert!(!xml.contains("\t"), "XML must not contain tabs");
}

#[test]
fn test_csv_structure_matches_sample() {
    let contributions = sample_contributions();
    let csv = generator::csv::build(&contributions, 2025, 12);

    let lines: Vec<&str> = csv.split("\r\n").collect();

    // Header line (unquoted)
    assert_eq!(
        lines[0],
        "LP;NR_PESEL;DOK_TOZSAMOSCI_RODZAJ;DOK_TOZSAMOSCI_SERIA_NUMER;UCZESTNIK_IDENTYFIKATOR_INFORMATYCZNY;NAZWISKO;IMIE;WARTOSC_PODST_PRACOWNIKA;WARTOSC_DODATK_PRACOWNIKA;WARTOSC_PODST_PRACODAWCY;WARTOSC_DODATK_PRACODAWCY;FLAGA_OBNIZENIE_SKL_PODST_PRACOWNIKA;ZA_MIESIAC;ZA_ROK;PZIF_RACH_PPK;ID_EPPK_UCZESTNIKA"
    );

    // First data row (all quoted, comma decimals, month unpadded)
    assert_eq!(
        lines[1],
        "\"1\";\"85032212342\";\"\";\"\";\"\";\"TESTOWA\";\"MARIA\";\"94,38\";\"0,00\";\"70,78\";\"0,00\";\"N\";\"12\";\"2025\";\"\";\"\""
    );

    // Verify CRLF
    assert!(csv.contains("\r\n"));

    // Verify semicolons in header
    assert_eq!(lines[0].matches(';').count(), 15);

    // Verify comma decimals (not dot)
    assert!(lines[1].contains("94,38"));
    assert!(!lines[1].contains("94.38"));

    // Verify month is unpadded
    assert!(lines[1].contains(";\"12\";"));
}

#[test]
fn test_csv_contains_last_name() {
    let contributions = sample_contributions();
    let csv = generator::csv::build(&contributions, 2025, 12);
    let lines: Vec<&str> = csv.split("\r\n").collect();

    // Check first row has TESTOWA in it
    assert!(lines[1].contains("TESTOWA"));
    assert!(lines[2].contains("PRZYKLADOWA "));
}

#[test]
fn test_zip_creates_valid_output() {
    let xml = "<PPK><WERSJA>test</WERSJA></PPK>";
    let csv = "LP;NR_PESEL\r\n\"1\";\"12345678901\"";
    let result = generator::zip::build(xml, csv).unwrap();

    assert!(!result.zip_bytes.is_empty());
    assert!(result.xml_filename.starts_with("SKLADKA_"));
    assert!(result.xml_filename.ends_with(".xml"));
    assert!(result.csv_filename.starts_with("SKLADKA_"));
    assert!(result.csv_filename.ends_with(".csv"));

    // Verify it's a valid ZIP
    let reader = std::io::Cursor::new(&result.zip_bytes);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    assert_eq!(archive.len(), 2);
}
