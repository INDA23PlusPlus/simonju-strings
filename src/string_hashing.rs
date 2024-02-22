const PRIME: u128 = 769;
const MODULO: u128 = 12_055_296_811_267; // more digits <=> more test cases passed
const SIZE: usize = 300_000;

const POWERS: [u128; SIZE] = {
    let mut powers = [1; SIZE];

    let mut i = 1;
    while i < SIZE {
        powers[i] = (powers[i - 1] * PRIME) % MODULO;
        
        i += 1;
    }

    powers
};

static mut HASHES: [u128; SIZE + 1] = [0; SIZE + 1];

pub fn string_hashing() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    let string = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };
    let _queries = lines.next();

    for (i, byte) in string.bytes().enumerate() {
        unsafe { HASHES[i + 1] = (((HASHES[i] * PRIME) % MODULO) + byte as u128) % MODULO};
    }

    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_ascii_whitespace();

        let left = unsafe { parts.next().unwrap_unchecked().parse::<usize>().unwrap_unchecked() };
        let right = unsafe { parts.next().unwrap_unchecked().parse::<usize>().unwrap_unchecked() };

        let hash = unsafe { ((HASHES[right].wrapping_sub((HASHES[left] * POWERS[right - left]) % MODULO)).wrapping_add(MODULO)) % MODULO};

        println!("{hash}");
    }
}