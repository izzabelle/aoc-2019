const INPUT: std::ops::Range<usize> = 183564..657474;

// returns true if all digits are ascending or equal
fn check_ascending(password: &usize) -> bool {
    let password = password.to_string();
    let mut tmp_byte: &u8 = &password.as_bytes()[0];

    for byte in password.as_bytes() {
        if byte >= tmp_byte {
            tmp_byte = byte;
        } else {
            return false;
        }
    }

    return true;
}

// check fo find repeating digits in password
fn check_double(password: &usize) -> bool {
    let password = password.to_string();
    let mut tmp_byte: &u8 = &255;

    for byte in password.as_bytes() {
        if byte == tmp_byte {
            return true;
        } else {
            tmp_byte = byte;
        }
    }

    return false;
}

// check fo find repeating digits in password
fn check_double_part_two(password: &usize) -> bool {
    let password = password.to_string();
    let mut map: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();

    password.as_bytes().iter().for_each(|byte| match map.get_mut(byte) {
        None => {
            map.insert(*byte, 1);
        }
        Some(count) => {
            *count += 1;
        }
    });

    let mut result = false;
    map.iter().for_each(|(_, count)| {
        if count == &2 {
            result = true;
        }
    });

    result
}

pub fn part_one() {
    let timer = std::time::Instant::now();
    let mut count = 0;
    for password in INPUT {
        if check_ascending(&password) && check_double(&password) {
            count += 1;
        }
    }
    println!("result: {}", count);
    println!("execution time: {}ms", timer.elapsed().as_millis());
}

pub fn part_two() {
    let timer = std::time::Instant::now();
    let mut count = 0;
    for password in INPUT {
        if check_ascending(&password) && check_double_part_two(&password) {
            count += 1;
        }
    }
    println!("result: {}", count);
    println!("execution time: {}ms", timer.elapsed().as_millis());
}
