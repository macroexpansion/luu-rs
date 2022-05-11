use std::fs::{File, OpenOptions};
use std::io::{copy, BufReader, BufWriter, Result};

pub fn file_copy(src_file_path: &str, dst_file_path: &str) -> Result<()> {
    let src_file = File::open(src_file_path)?;
    let dst_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(dst_file_path)?;
    let mut src: BufReader<File> = BufReader::new(src_file);
    let mut dst: BufWriter<File> = BufWriter::new(dst_file);
    copy(&mut src, &mut dst)?;
    Ok(())
}

pub fn clipboard_copy(src_file_path: &str) -> Result<()> {
    unimplemented!();
}
