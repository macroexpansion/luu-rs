use std::{error::Error, fs::File, io::Read};

// use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cli_clipboard::{get_contents, set_contents};

pub fn clipboard_copy(src_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut src_file = File::open(src_file_path)?;
    // let mut ctx = ClipboardContext::new()?;

    let mut buffer: String = String::new();
    src_file.read_to_string(&mut buffer)?;

    println!("{:?}", get_contents()?);
    // println!("{:?}", buffer.trim_end().to_string());
    set_contents(buffer.trim_end().to_string())?;

    println!("{:?}", get_contents()?);
    Ok(())
}
