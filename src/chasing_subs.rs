const RESET: [u8; 27] = [u8::MAX; 27];
static mut ARRAY: [u8; 27] = RESET;

static mut PATTERN: [u8; 250_000] = [0; 250_000];

pub fn chasing_subs() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    let string = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };
    let pattern = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };

    let mut longest_sequence = 0;
    let mut sequence_counter = 0;

    let mut pattern_iter = pattern.bytes().enumerate();

    unsafe { ARRAY[pattern_iter.next().unwrap().1 as usize - 97] = 0 }

    let mut alphabet_counter = 1;

    while let Some((i, byte)) = pattern_iter.next() {
        if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
            unsafe { ARRAY[byte as usize - 97] = alphabet_counter };
            alphabet_counter += 1;
        }

        unsafe { PATTERN[i] = ARRAY[byte as usize - 97] };

        if unsafe { PATTERN[i] == PATTERN[i - 1] } {
            sequence_counter += 1;
        } else {
            longest_sequence = longest_sequence.max(sequence_counter);
            sequence_counter = 0;
        }
    }

    let mut matches = true;
    let mut match_index = 0;
    let mut match_count = 0;

    let mut iter = (0..string.len() - pattern.len() + 1).into_iter();

    while let Some(i) = iter.next() {
        unsafe { ARRAY = RESET };
        alphabet_counter = 1;

        let substring = unsafe { string.get_unchecked(i..i + pattern.len()) };

        unsafe { ARRAY[ *substring.as_bytes().get_unchecked(0) as usize - 97] = 0 };

        for (j, byte) in substring.bytes().enumerate().skip(1) {
            if unsafe { ARRAY[byte as usize - 97] } == u8::MAX {
                unsafe { ARRAY[byte as usize - 97] = alphabet_counter };
                alphabet_counter += 1;
            }

            if unsafe { ARRAY[byte as usize - 97] != PATTERN[j] } {
                matches = false;
                break
            }
        }

        if matches {
            match_index = i;
            match_count += 1;

            if longest_sequence > 0 { iter.nth(longest_sequence - 1); }
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
=> essa

treetreetreetree
wood
=> 3

oranges
apples
=> 0

archipelago
submarine
=> 2
*/ 