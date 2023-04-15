use anyhow::Result;
use md5::{Digest, Md5};
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let filenames: Vec<&str> = args[1..].iter().map(AsRef::as_ref).collect();
    process_files(&filenames)?;

    Ok(())
}

fn print_usage() {
    println!("Usage: md5_hasher <file1> <file2> ...");
}

fn process_files(filenames: &[&str]) -> Result<()> {
    filenames.par_iter().try_for_each(|filename| {
        process_file(filename).map_err(|e| {
            eprintln!("Error processing file {}: {:?}", filename, e);
            e
        })
    })
}

fn process_file(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Md5::new();
    let mut buffer = [0; 1024];

    loop {
        let read_bytes = reader.read(&mut buffer)?;

        if read_bytes == 0 {
            break;
        }

        hasher.update(&buffer[..read_bytes]);
    }

    let md5_hash = format!("{:X}", hasher.finalize());

    let output_filename = format!("{}.md5", path.file_name().unwrap().to_string_lossy());
    let mut output_file = File::create(output_filename)?;
    writeln!(&mut output_file, "{}", md5_hash)?;

    Ok(())
}
