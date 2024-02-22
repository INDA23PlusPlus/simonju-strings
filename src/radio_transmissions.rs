pub fn radio_transmissions() {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut lines = std::io::BufRead::lines(reader);

    let l = unsafe {
        lines.next()
            .unwrap_unchecked()
            .unwrap_unchecked()
            .parse::<usize>()
            .unwrap_unchecked()
    };

    let s = unsafe {
        lines.next()
            .unwrap_unchecked()
            .unwrap_unchecked()
    };

    for i in 1..l {
        let a = unsafe { s.get_unchecked(0..i) };

        let mut matches = true;

        for j in (i..l).step_by(i) {
            let b = unsafe { s.get_unchecked(j..(j + i).min(l)) };

            if !a.starts_with(b) { matches = false; break }
        }

        if matches { return println!("{i}") }
    }

    println!("{l}")
}