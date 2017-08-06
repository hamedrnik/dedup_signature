fn rot(x: u32, k: usize) -> u32 {
    ((x << k) | (x >> (32 - k)))
}

fn mix(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = a.wrapping_sub(c);
    a ^= rot(c, 4);
    c = c.wrapping_add(b);
    b = b.wrapping_sub(a);
    b ^= rot(a, 6);
    a = a.wrapping_add(c);
    c = c.wrapping_sub(b);
    c ^= rot(b, 8);
    b = b.wrapping_add(a);
    a = a.wrapping_sub(c);
    a ^= rot(c, 16);
    c = c.wrapping_add(b);
    b = b.wrapping_sub(a);
    b ^= rot(a, 19);
    a = a.wrapping_add(c);
    c = c.wrapping_sub(b);
    c ^= rot(b, 4);
    b = b.wrapping_add(a);

    (a, b, c)
}

fn do_final(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    c ^= b;
    c = c.wrapping_sub(rot(b, 14));
    a ^= c;
    a = a.wrapping_sub(rot(c, 11));
    b ^= a;
    b = b.wrapping_sub(rot(a, 25));
    c ^= b;
    c = c.wrapping_sub(rot(b, 16));
    a ^= c;
    a = a.wrapping_sub(rot(c, 4));
    b ^= a;
    b = b.wrapping_sub(rot(a, 14));
    c ^= b;
    c = c.wrapping_sub(rot(b, 24));

    c
}

pub fn lookup3(k: &Vec<u32>, offset: u32, mut length: usize, initval: u32) -> u32 {
    // Set up the internal state
    let mut a: u32 = 0xdeadbeef;
    let mut b: u32 = 0xdeadbeef;
    let mut c: u32 = 0xdeadbeef;
    a = a.wrapping_add(((length as u32) << 2)).wrapping_add(initval);
    b = b.wrapping_add(((length as u32) << 2)).wrapping_add(initval);
    c = c.wrapping_add(((length as u32) << 2)).wrapping_add(initval);

    //------------------------------------------------- handle most of the key
    let mut i = offset as usize;
    while length > 3 {
        a = a.wrapping_add(k[i]);
        b = b.wrapping_add(k[i + 1]);
        c = c.wrapping_add(k[i + 2]);

        let (d, e, f) = mix(a, b, c);
        a = d;
        b = e;
        c = f;

        length -= 3;
        i += 3;
    }

    //--------------------------- handle the last 3 u32's and report the result

    match length {
        3 => {
            c += k[i + 2];
            b += k[i + 1];
            a += k[i + 0];
            do_final(a, b, c)
        }
        2 => {
            b += k[i + 1];
            a += k[i + 0];
            do_final(a, b, c)
        }
        1 => {
            a += k[i + 0];
            do_final(a, b, c)
        }
        _ => c,
    }
}

pub fn lookup3_u64(k: &Vec<u32>, offset: u32, mut length: usize, initval: u64) -> u64 {
    // Set up the internal state
    let mut a: u32 = 0xdeadbeef;
    let mut b: u32 = 0xdeadbeef;
    let mut c: u32 = 0xdeadbeef;
    a = a.wrapping_add(((length as u32) << 2)).wrapping_add(
        (initval >> 32) as u32,
    );
    b = b.wrapping_add(((length as u32) << 2)).wrapping_add(
        (initval >> 32) as u32,
    );
    c = c.wrapping_add(((length as u32) << 2)).wrapping_add(
        (initval >> 32) as u32,
    );

    //------------------------------------------------- handle most of the key
    let mut i = offset as usize;
    while length > 3 {
        a = a.wrapping_add(k[i]);
        b = b.wrapping_add(k[i + 1]);
        c = c.wrapping_add(k[i + 2]);

        let (d, e, f) = mix(a, b, c);
        a = d;
        b = e;
        c = f;

        length -= 3;
        i += 3;
    }

    //--------------------------- handle the last 3 u32's and report the result

    match length {
        3 => {
            c += k[i + 2];
            b += k[i + 1];
            a += k[i + 0];
            do_final(a, b, c) as u64 + ((b as u64) << 32)
        }
        2 => {
            b += k[i + 1];
            a += k[i + 0];
            do_final(a, b, c) as u64 + ((b as u64) << 32)
        }
        1 => {
            a += k[i + 0];
            do_final(a, b, c) as u64 + ((b as u64) << 32)
        }
        _ => c as u64 + ((b as u64) << 32),
    }
}


#[cfg(test)]
mod test {
    use hash::*;

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
        let mut a = vec![0; s.chars().count()];

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
        let mut a = vec![0; s.chars().count()];

        for (i, c) in s.chars().enumerate() {
            a[i] = c as u32;
            let len = i + 1;
            let hash = lookup3(&a, 0, len as usize, (i * 12345) as u32);
            assert_eq!(hashes[i] as u32, hash);
        }
    }

    #[test]
    fn test_equals_lookup3_u64() {
        let hashes: [u64; 11] = [
            0xdeadbef3c4c20dd5,
            0xdeadbf5c2b0533e5,
            0xdeadbf601977d67f,
            0x8d29c8f6a0056e90,
            0xb1042c1b7df4026b,
            0xa88f28eacfe50268,
            0xb5349593ca06e1de,
            0x478e5eebd73775f2,
            0xd7ed36c3d2ab23a3,
            0x46e0336bb914de3c,
            0xf8a1d3dcc94dc067,
        ];

        let s = "hello world";
        let mut a = vec![0; s.chars().count()];

        for (i, c) in s.chars().enumerate() {
            a[i] = c as u32;
            let len = i + 1;
            let hash = lookup3_u64(&a, 0, len as usize, (i * 12345) as u64);
            assert_eq!(hashes[i], hash);
        }
    }
}
