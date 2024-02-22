const RESET: [u8; 27] = [u8::MAX; 27];
static mut ARRAY: [u8; 27] = RESET;

static mut PATTERN: [u8; 250_000] = [0; 250_000];

pub fn chasing_subs() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    let string = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };
    let pattern = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };

    let mut counter = 0;

    for (i, byte) in pattern.bytes().enumerate() {
        if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
            unsafe { ARRAY[byte as usize - 97] = counter };
            counter += 1;
        }

        unsafe { PATTERN[i] = ARRAY[byte as usize - 97] };
    }

    // println!("PATTERN: {:?}", unsafe { PATTERN.get_unchecked(0..pattern.len()) });

    let mut matches = true;
    let mut match_index = 0;
    let mut match_count = 0;

    let mut iter = (0..string.len() - pattern.len() + 1).into_iter();

    while let Some(i) = iter.next() {
        unsafe { ARRAY = RESET };
        counter = 1;

        let substring = unsafe { string.get_unchecked(i..i + pattern.len()) };

        // println!("substring #{i}");

        unsafe { ARRAY[ *substring.as_bytes().get_unchecked(0) as usize - 97] = 0 };

        for (j, byte) in substring.bytes().enumerate().skip(1) {
            if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
                unsafe { ARRAY[byte as usize - 97] = counter };
                counter += 1;
            }

            if unsafe { ARRAY[byte as usize - 97] != PATTERN[j] } {
                matches = false;

                // println!("{} != {}", unsafe { PATTERN[j] }, unsafe { ARRAY[byte as usize - 97] });
                break
            }

            // println!("{} == {}", unsafe { PATTERN[j] }, unsafe { ARRAY[byte as usize - 97] });
        }

        if matches {
            match_index = i;
            match_count += 1;
        }

        matches = true;
    }

    match match_count {
        1 => println!("{}", unsafe { string.get_unchecked(match_index..match_index + pattern.len()) }),
        i => println!("{i}"),
    }
}


pub fn chasing_subs_old() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    let string = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };
    let pattern = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };

    let mut counter = 0;

    for (i, byte) in pattern.bytes().enumerate() {
        if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
            unsafe { ARRAY[byte as usize - 97] = counter };
            counter += 1;
        }

        unsafe { PATTERN[i] = ARRAY[byte as usize - 97] };
    }

    println!("PATTERN: {:?}", unsafe { PATTERN.get_unchecked(0..pattern.len()) });

    let mut matches = true;
    let mut match_index = 0;
    let mut match_count = 0;

    for i in 0..string.len() - pattern.len() + 1 {
        unsafe { ARRAY = RESET };
        counter = 1;

        let substring = unsafe { string.get_unchecked(i..i + pattern.len()) };

        println!("substring #{i}");

        unsafe { ARRAY[ *substring.as_bytes().get_unchecked(0) as usize - 97] = 0 };

        for (j, byte) in substring.bytes().enumerate().skip(1) {
            if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
                unsafe { ARRAY[byte as usize - 97] = counter };
                counter += 1;
            }

            if unsafe { ARRAY[byte as usize - 97] != PATTERN[j] } {
                matches = false;

                println!("{} != {}", unsafe { PATTERN[j] }, unsafe { ARRAY[byte as usize - 97] });
                break
            }

            println!("{} == {}", unsafe { PATTERN[j] }, unsafe { ARRAY[byte as usize - 97] });
        }

        if matches {
            match_index = i;
            match_count += 1;
        }

        matches = true;
    }

    match match_count {
        1 => println!("{}", unsafe { string.get_unchecked(match_index..match_index + pattern.len()) }),
        i => println!("{i}"),
    }
}

/*
secretmessage
boot
*/ 