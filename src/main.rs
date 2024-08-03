use std::env;
use std::fmt;
use std::fs;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
enum ConversionError {
    InvalidFileType(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::InvalidFileType(filename) =>
                write!(f, "Invalid file type: {}", filename),
        }
    }
}

impl Error for ConversionError {}

#[derive(debug)]
enum FileType {
    Yaml,
    Json
}

fn determine_file_type(filename: &str) -> Result<FileType, ConversionError> {
    let path = Path::new(filename);
    if let Some(extension) = path.extension() {
        match extension.to_str() {
            Some("yml") | Some("yaml") => Ok(FileType::Yaml),
            Some("json") => Ok(FileType::Json),
            _ => Err(ConversionError::InvalidFileType(filename.to_string())),
        }
    } else {
        Err(ConversionError::InvalidFileType(filename.to_string()))
    }
}

fn convert_file(filename: &str) -> Result<(), ConversionError> {
    match determine_file_type(filename)? {
        FileType::Yaml => convert_yaml_to_json(filename),
        FileType::Json => convert_json_to_yaml(filename)
    }
}

fn convert_yaml_to_json(filename: &str) -> Result<(), ConversionError> {
    Ok(())
}

fn convert_json_to_yaml(filename: &str) -> Result<(), ConversionError> {
    Ok(())
}

fn file_debug(filename: &str) {
    println!("Input: {}", filename);
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("Content: \n{}", content)
        },
        Err(e) => {println!("Error: {}", e)}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }
    let input_file = &args[1];
    convert_file(input_file);
    file_debug(input_file);
    Ok(())
}
