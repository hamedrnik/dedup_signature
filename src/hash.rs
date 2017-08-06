fn rot(x: u32, k: usize) -> u32 {
    ((x << k) | (x >> (32 - k)))
}

fn rot_u64(x: u64, k: usize) -> u64 {
    ((x << k) | (x >> (64 - k)))
}

fn mix(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = a.wrapping_sub(c);  a ^= rot(c, 4);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot(a, 6);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot(b, 8);  b = b.wrapping_add(a);
    a = a.wrapping_sub(c);  a ^= rot(c,16);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot(a,19);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot(b, 4);  b = b.wrapping_add(a);
    
    (a, b, c)
}

fn mix_u64(mut a: u64, mut b: u64, mut c: u64) -> (u64, u64, u64) {
    a = a.wrapping_sub(c);  a ^= rot_u64(c, 4);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot_u64(a, 6);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot_u64(b, 8);  b = b.wrapping_add(a);
    a = a.wrapping_sub(c);  a ^= rot_u64(c,16);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot_u64(a,19);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot_u64(b, 4);  b = b.wrapping_add(a);
    
    (a, b, c)
}

fn do_final(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    c ^= b; c = c.wrapping_sub(rot(b,14));
    a ^= c; a = a.wrapping_sub(rot(c,11));
    b ^= a; b = b.wrapping_sub(rot(a,25));
    c ^= b; c = c.wrapping_sub(rot(b,16));
    a ^= c; a = a.wrapping_sub(rot(c,4)); 
    b ^= a; b = b.wrapping_sub(rot(a,14));
    c ^= b; c = c.wrapping_sub(rot(b,24));
    
    c
}

fn do_final_u64(mut a: u64, mut b: u64, mut c: u64) -> u64 {
    c ^= b; c = c.wrapping_sub(rot_u64(b,14));
    a ^= c; a = a.wrapping_sub(rot_u64(c,11));
    b ^= a; b = b.wrapping_sub(rot_u64(a,25));
    c ^= b; c = c.wrapping_sub(rot_u64(b,16));
    a ^= c; a = a.wrapping_sub(rot_u64(c,4)); 
    b ^= a; b = b.wrapping_sub(rot_u64(a,14));
    c ^= b; c = c.wrapping_sub(rot_u64(b,24));
    
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

pub fn lookup3_u64(k: &Vec<u64>, offset: u64, mut length: usize, initval: u64) -> u64 {
    // Set up the internal state
    let mut a: u64 = 0xdeadbeef;
    let mut b: u64 = 0xdeadbeef;
    let mut c: u64 = 0xdeadbeef;
    a = a.wrapping_add(((length as u64) << 2)).wrapping_add(initval);
    b = b.wrapping_add(((length as u64) << 2)).wrapping_add(initval);
    c = c.wrapping_add(((length as u64) << 2)).wrapping_add(initval);

    //------------------------------------------------- handle most of the key
    let mut i = offset as usize;
    while length > 3 {
        a = a.wrapping_add(k[i]);
        b = b.wrapping_add(k[i + 1]);
        c = c.wrapping_add(k[i + 2]);

        let (d, e, f) = mix_u64(a, b, c);
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
            do_final_u64(a, b, c)
        }
        2 => {
            b += k[i + 1];
            a += k[i + 0];
            do_final_u64(a, b, c)
        }
        1 => {
            a += k[i + 0];
            do_final_u64(a, b, c)
        }
        _ => c,
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
            0xbe03c26ae6c2235e,
            0xce1eadf96d51f82a,
            0x22e7362910a739ee,
            0xf2731a8d66d0f7cf,
            0x151ecadec05b6ea0,
            0x6923619ad9148cb6,
            0x284e28502f4b70af,
            0xb9494bf1d99ee2fe,
            0x3e72a8979193e745,
            0x5eb84a3df830ac14,
            0xe1123a81b8e2ec77,
        ];

        let s = "hello world";
        let mut a = vec![0; s.chars().count()];

        for (i, c) in s.chars().enumerate() {
            a[i] = c as u64;
            let len = i + 1;
            let hash = lookup3_u64(&a, 0, len as usize, (i * 12345) as u64);
            assert_eq!(hashes[i] as u64, hash);
        }
    }
}
