use rand::Rng;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rhred <file_or_directory>");
        std::process::exit(1);
    }

    let target_path = Path::new(&args[1]);

    if target_path.is_file() {
        shred_file(target_path)?;
    } else if target_path.is_dir() {
        shred_directory(target_path)?;
    } else {
        eprintln!("Invalid file or directory path");
        std::process::exit(1);
    }

    Ok(())
}

fn shred_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file_size = fs::metadata(path)?.len();
    let mut file = BufWriter::new(File::create(path)?);

    // Shred the file by overwriting it with random data
    let mut rng = rand::thread_rng();
    let mut buffer = vec![0u8; 1024 * 1024];
    let mut remaining_bytes = file_size;
    while remaining_bytes > 0 {
        let bytes_to_write = std::cmp::min(buffer.len() as u64, remaining_bytes) as usize;
        rng.fill_bytes(&mut buffer[..bytes_to_write]);
        file.write_all(&buffer[..bytes_to_write])?;
        remaining_bytes -= bytes_to_write as u64;
    }

    file.flush()?;
    Ok(())
}

fn shred_directory(path: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            shred_file(&entry_path)?;
        } else if entry_path.is_dir() {
            shred_directory(&entry_path)?;
        }
    }

    Ok(())
}