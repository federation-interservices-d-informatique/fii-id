extern crate fii_id;
#[test]
pub fn checknum_works_fine() {
    assert_eq!(fii_id::utils::functions::checknum(String::from("8")), "00008");
    assert_eq!(fii_id::utils::functions::checknum(String::from("88")), "00088");
    assert_eq!(fii_id::utils::functions::checknum(String::from("888")), "00888");
    assert_eq!(fii_id::utils::functions::checknum(String::from("8888")), "08888");
    assert_eq!(fii_id::utils::functions::checknum(String::from("88888")), "88888");
}