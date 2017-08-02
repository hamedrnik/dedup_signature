fn rot(x: i32, k: usize) -> i32 {
    ((x << k) | ((x as u32) >> (32 - k)) as i32)
}

fn mix(mut a: i32, mut b: i32, mut c: i32) -> (i32, i32, i32) {
    a = a.wrapping_sub(c);  a ^= rot(c, 4);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot(a, 6);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot(b, 8);  b = b.wrapping_add(a);
    a = a.wrapping_sub(c);  a ^= rot(c,16);  c = c.wrapping_add(b);
    b = b.wrapping_sub(a);  b ^= rot(a,19);  a = a.wrapping_add(c);
    c = c.wrapping_sub(b);  c ^= rot(b, 4);  b = b.wrapping_add(a);
    
    (a, b, c)
}

fn do_final(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    c ^= b; c = c.wrapping_sub(rot(b,14));
    a ^= c; a = a.wrapping_sub(rot(c,11));
    b ^= a; b = b.wrapping_sub(rot(a,25));
    c ^= b; c = c.wrapping_sub(rot(b,16));
    a ^= c; a = a.wrapping_sub(rot(c,4)); 
    b ^= a; b = b.wrapping_sub(rot(a,14));
    c ^= b; c = c.wrapping_sub(rot(b,24));
    
    c
}

pub fn lookup3(k: &[i32], offset: i32, mut length: usize, initval: i32) -> i32 {
    // Set up the internal state
    let mut a: i32 = 0xdeadbeef + ((length as i32) << 2) as i32 + initval;
    let mut b: i32 = 0xdeadbeef + ((length as i32) << 2) as i32 + initval;
    let mut c: i32 = 0xdeadbeef + ((length as i32) << 2) as i32 + initval;

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

    //------------------------------------------- handle the last 3 uint32_t's

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
