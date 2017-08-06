// De-duplication Signature generates a hash of textual fields for de-duplication.
// Copyright 2016-2017 Hamed Ramezanian Nik

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
