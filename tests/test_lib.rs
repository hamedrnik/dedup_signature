// Deduplication Signature generates a hash of textual fields for deduplication.
// Currently it supports only Text Profile Signature.
// Copyright (C) 2016  Hamed Ramezanian Nik

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
extern crate yaml_rust;

use yaml_rust::YamlLoader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use dedup_signature::generator::*;

fn get_wikipedia_article(lang: &str) -> (String, String) {
    let mut filename = String::from("liberty_article_from_");
    filename.push_str(lang);
    filename.push_str("_wikipedia.yaml");

    let path = env::current_dir().unwrap().join("tests/fixtures").join(
        filename,
    );

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let docs = YamlLoader::load_from_str(&s).unwrap();

    let doc = &docs[0];
    let article = String::from(doc[":article"].as_str().unwrap());
    let signature = String::from(doc[":signature"].as_str().unwrap());

    (article, signature)
}


#[test]
fn test_en_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("en");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_de_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("de");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_es_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("es");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_fr_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("fr");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_it_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("it");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}

#[test]
fn test_pt_lang() {
    let profile_generator = TextProfileSignature { ..TextProfileSignature::default() };
    let (article, expected_sign) = get_wikipedia_article("pt");
    let sign = profile_generator.generate_sign(&article);

    assert_eq!(expected_sign, sign);
}
