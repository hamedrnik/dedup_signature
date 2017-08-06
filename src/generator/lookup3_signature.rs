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

use hash::*;

pub struct Lookup3Signature {
    pub seed: u64,
}

impl Lookup3Signature {
    pub fn default() -> Self {
        Lookup3Signature { seed: 12345 }
    }

    pub fn generate_sign(&self, text: &str) -> String {
        let mut a = vec![0; text.chars().count()];
        let mut hash = self.seed as u32;

        for (i, c) in text.chars().enumerate() {
            a[i] = c as u32;
            let len = i + 1;
            hash = lookup3(&a, 0, len as usize, hash);
        }

        format!("{:08x}", hash)
    }

    pub fn generate_sign_64(&self, text: &str) -> String {
        let mut a = vec![0; text.chars().count()];
        let mut hash = self.seed;

        for (i, c) in text.chars().enumerate() {
            a[i] = c as u32;
            let len = i + 1;
            hash = lookup3_u64(&a, 0, len as usize, hash);
        }

        format!("{:016x}", hash)
    }
}
