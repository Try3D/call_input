pub fn get_string(s: &str) -> String {
    println!("{s}");

    let mut n = String::new();

    std::io::stdin().read_line(&mut n).expect("N/A");

    return n.trim().to_string();
}

pub fn get_char(s: &str) -> char {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_bool(s: &str) -> bool {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_i8(s: &str) -> i8 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_u8(s: &str) -> u8 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_i16(s: &str) -> i16 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_u16(s: &str) -> u16 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_i32(s: &str) -> i32 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_u32(s: &str) -> u32 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_i64(s: &str) -> i64 {
    loop {
        let n = get_string(s);
        let n: i64 = match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return n;
    }
}

pub fn get_u64(s: &str) -> u64 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_i128(s: &str) -> i128 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_u128(s: &str) -> u128 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_f32(s: &str) -> f32 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_f64(s: &str) -> f64 {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_usize(s: &str) -> usize {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

pub fn get_isize(s: &str) -> isize {
    loop {
        let n = get_string(s);
        return match n.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
