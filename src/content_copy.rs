use std::{error::Error, fs::File, io::Read};

use cli_clipboard::{get_contents, set_contents};

pub fn clipboard_copy(src_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut src_file = File::open(src_file_path)?;
    let mut buffer: String = String::new();
    src_file.read_to_string(&mut buffer)?;
    set_contents(buffer.trim_end().to_string())?;
    Ok(())
}
