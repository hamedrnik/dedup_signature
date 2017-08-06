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
            a[i] = c as u64;
            let len = i + 1;
            hash = lookup3_u64(&a, 0, len as usize, hash);
        }

        format!("{:016x}", hash)
    }
}
