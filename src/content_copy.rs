// use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_ext::prelude::*;
use clipboard_ext::x11_bin::ClipboardContext;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn clipboard_copy(src_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut src_file = File::open(src_file_path)?;
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut buffer: String = String::new();
    src_file.read_to_string(&mut buffer)?;
    ctx.set_contents(buffer.trim_end().to_string())?;
    // println!("{:?}", ctx.get_contents()?);
    Ok(())
}
