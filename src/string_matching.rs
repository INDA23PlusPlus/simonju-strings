pub fn string_matching() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    while let Some(Ok(pattern)) = lines.next() {
        let string: String = unsafe { lines.next().unwrap_unchecked().unwrap_unchecked() };
        
        for i in 0..(string.len() - pattern.len() + 1) {
            if pattern == unsafe { string.get(i..(i + pattern.len()).min(string.len())).unwrap_unchecked() } {
                print!("{i} ");
            }
        }

        println!();
    }
}