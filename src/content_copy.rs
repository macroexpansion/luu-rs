use std::{error::Error, fs::File, io::Read};

use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn clipboard_copy(src_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut src_file = File::open(src_file_path)?;
    let mut ctx = ClipboardContext::new()?;

    let mut buffer: String = String::new();
    src_file.read_to_string(&mut buffer)?;

    ctx.set_contents(buffer.trim_end().to_string())?;

    let _resp = ctx.get_contents()?;
    // println!("{:?}", _resp);
    println!("Copied to clipboard");

    Ok(())
}
