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

use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub struct TextProfileSignature {
    pub min_token_length: usize,
    pub quant_rate: f32,
}

struct Token {
    count: u16,
    term: String,
}

impl TextProfileSignature {
    pub fn default() -> Self {
        TextProfileSignature {
            min_token_length: 2,
            quant_rate: 0.01,
        }
    }

    pub fn generate_sign(&self, text: &str) -> String {
        // remove all characters except letters and digits,
        // and bring all characters to lower case
        // split the text into tokens (all consecutive non-whitespace characters)
        // discard tokens equal or shorter than MIN_TOKEN_LEN (default 2 characters)
        let mut current_token = String::new();
        let mut max_freq = 0;
        let mut tokens = HashMap::new();

        for c in text.chars() {
            if c.is_alphabetic() || c.is_digit(10) {
                current_token.push_str(&c.to_lowercase().collect::<String>().clone());
            } else {
                if current_token.chars().count() > 0 {
                    if current_token.chars().count() > self.min_token_length {
                        if !tokens.contains_key(&current_token) {
                            let tok = Token {
                                count: 0,
                                term: current_token.clone(),
                            };
                            tokens.insert(current_token.clone(), tok);
                        }

                        tokens.get_mut(&current_token).unwrap().count += 1;

                        if tokens.get(&current_token).unwrap().count > max_freq {
                            max_freq = tokens.get(&current_token).unwrap().count;
                        }
                    }
                    current_token = String::from("");
                }
            }
        }

        //  Check the last token
        if current_token.chars().count() > self.min_token_length {
            // Add it
            if !tokens.contains_key(&current_token) {
                let tok = Token {
                    count: 0,
                    term: current_token.clone(),
                };
                tokens.insert(current_token.clone(), tok);
            }

            tokens.get_mut(&current_token).unwrap().count += 1;

            if tokens.get(&current_token).unwrap().count > max_freq {
                max_freq = tokens.get(&current_token).unwrap().count;
            }
        }

        // calculate the QUANT value
        let mut quant = (max_freq as f32 * self.quant_rate).round() as u16;

        if quant < 2 {
            if max_freq > 1 {
                quant = 2;
            } else {
                quant = 1;
            }
        }

        // round down the counts of tokens to the nearest multiple of QUANT
        // tokens, which frequency after quantization falls below QUANT, are discarded
        let mut quantized_tokens: Vec<Token> =
            tokens.iter().fold(Vec::new(), |mut memo, (_, val)| {
                let quantized_count = (val.count / quant) * quant;

                if quantized_count >= quant {
                    let item = Token {
                        count: quantized_count,
                        term: val.term.clone(),
                    };
                    memo.push(item);
                }

                memo
            });

        // sort the list of tokens by decreasing frequency
        quantized_tokens.sort_by(|x, y| {
            (y.count, &x.term).partial_cmp(&(x.count, &y.term)).unwrap()
        });

        // create a list of tokens and their quantized frequency,
        // separated by spaces, in the order of decreasing frequency
        let quantized_frequency_vec: Vec<String> = quantized_tokens
            .iter()
            .map(|a| format!("{} {}", a.term, a.count))
            .collect();

        let quantized_frequency_str = quantized_frequency_vec.join("\n");

        let mut hasher = Md5::new();
        hasher.input_str(&quantized_frequency_str);

        hasher.result_str()
    }
}
