extern crate dedup_signature;

use dedup_signature::generator::lookup3_signature::*;

#[test]
fn test_en_word() {
    let profile_generator = Lookup3Signature { ..Lookup3Signature::default() };
    let expected_sign = "170856d1";
    let article = "hello world";
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_en_word_u64_sign() {
    let profile_generator = Lookup3Signature { ..Lookup3Signature::default() };
    let expected_sign = "f6d312543873dc23";
    let article = "hello world";
    let sign = profile_generator.generate_sign_64(&article);

    assert_eq!(expected_sign, sign);
}
