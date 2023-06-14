use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let directory = "C:/Users/aweso/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/";
     let files = fs::read_dir(directory)?;

     for file in files {
        let file = file?.path();
        let extension = file.extension();
        // Check if the file is an HTML file
        if extension == Some(std::ffi::OsStr::new("html")) {
            println!("{}", file.file_name().unwrap().to_str().unwrap());
            modify_html_file(&file)?;
        }
    }
    Ok(())
}

fn modify_html_file(file_path: &Path) -> io::Result<()> {
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    // Create a temporary file to write the modified content
    let temp_file_path = file_path.with_extension("tmp");
    let temp_file = File::create(&temp_file_path)?;
    let mut writer = BufWriter::new(temp_file);

    for line in reader.lines() {
        let line = line?;

        if line.contains("var default_theme = window.matchMedia(\"(prefers-color-scheme: dark)\").matches ? \"navy\" : \"light\";") {
            writeln!(writer, "var default_theme=\"light\";")?;
        } else {
            writeln!(writer, "{}", line)?;
        }
    }

    writer.flush()?;
    drop(writer);
    // Replace the input file with the temporary file
    fs::rename(temp_file_path, file_path)?;
    Ok(())
}
