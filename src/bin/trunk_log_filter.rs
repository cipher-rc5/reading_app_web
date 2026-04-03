#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[cfg(not(target_arch = "wasm32"))]
fn is_emoji(ch: char) -> bool {
    matches!(
        ch as u32,
        0x2600..=0x27BF
            | 0x1F300..=0x1FAFF
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        for ch in line.chars() {
            if !is_emoji(ch) {
                write!(writer, "{ch}")?;
            }
        }
        writer.flush()?;
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // This binary is not meant to run in WASM context
    panic!("trunk_log_filter is a native-only binary");
}
