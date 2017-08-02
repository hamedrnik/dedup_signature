fn rot(x: u32, k: usize) -> u32 {
    ((x << k) | (x >> (32 - k)))
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

pub fn lookup3(k: &[u32], offset: u32, mut length: usize, initval: u32) -> u32 {
    // Set up the internal state
    let mut a: u32 = 0xdeadbeef + ((length as u32) << 2) + initval;
    let mut b: u32 = 0xdeadbeef + ((length as u32) << 2) + initval;
    let mut c: u32 = 0xdeadbeef + ((length as u32) << 2) + initval;

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
