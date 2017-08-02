extern crate dedup_signature;

use dedup_signature::generator::lookup3::*;

#[test]
fn test_equals_lookup3() {
    let hashes: [u32; 11] = [
        0xc4c20dd5,
        0x3ab04cc3,
        0xebe874a3,
        0x0e770ef3,
        0xec321498,
        0x73845e86,
        0x8a2db728,
        0x03c313bb,
        0xfe5b9199,
        0x95965125,
        0xcbc4e7c2,
    ];

    let s = "hello world";
    let mut a: [u32; 11] = [0; 11];

    for (i, c) in s.chars().enumerate() {
        a[i] = c as u32;
        let len = i + 1;
        let hash = lookup3(&a, 0, len as usize, (i * 12345) as u32);
        assert_eq!(hashes[i] as u32, hash);
    }
}

#[test]
fn test_equals_lookup3_persian_string() {
    let hashes: [u32; 9] = [
        0xddaa5545,
        0xeb6d1e8f,
        0xa3e01aa5,
        0xdc1b974d,
        0x11ab2013,
        0xf577bc81,
        0xb37516ae,
        0xe696e02d,
        0x27f779c5,
    ];

    let s = "سلام دنیا";
    let mut a: [u32; 9] = [0; 9];

    for (i, c) in s.chars().enumerate() {
        a[i] = c as u32;
        let len = i + 1;
        let hash = lookup3(&a, 0, len as usize, (i * 12345) as u32);
        assert_eq!(hashes[i] as u32, hash);
    }
}
