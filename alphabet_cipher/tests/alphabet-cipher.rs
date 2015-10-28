extern crate alphabet_cipher;

use alphabet_cipher::*;

#[test]
pub fn can_encode_given_a_secret_keyword() {
    assert_eq!("hmkbxebpxpmyllyrxiiqtoltfgzzv", encode("vigilance", "meetmeontuesdayeveningatseven"));
    assert_eq!("egsgqwtahuiljgs", encode("scones", "meetmebythetree"));
}

#[test]
pub fn can_decode_an_encrypted_message_given_the_secret_keyword() {
    assert_eq!("meetmeontuesdayeveningatseven", encode("vigilance", "hmkbxebpxpmyllyrxiiqtoltfgzzv"));
    assert_eq!("meetmebythetree", encode("scones", "egsgqwtahuiljgs"));
}

pub fn can_extract_the_secret_keyword_given_an_encrypted_message_and_the_original_message() {
    assert_eq!("vigilance", decipher("opkyfipmfmwcvqoklyhxywgeecpvhelzg", "thequickbrownfoxjumpsoveralazydog"));
    assert_eq!("scones", decipher("hcqxqqtqljmlzhwiivgbsapaiwcenmyu", "packmyboxwithfivedozenliquorjugs"));
}
