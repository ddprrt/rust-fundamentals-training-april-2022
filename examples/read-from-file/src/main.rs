// Boxed errors -> anyhow
// Enum error variants -> thiserror


use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn read_data_from_path(path: Option<String>) -> Result<u32, ReadDataError> {
    let mut file = File::open(path.unwrap_or("data.txt".to_string()))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let result: u32 = buf.parse()?;
    Ok(result)
}

#[derive(Debug)]
enum ReadDataError {
    ParseError(ParseIntError),
    IoError(std::io::Error)
}

impl std::error::Error for ReadDataError {}

impl std::fmt::Display for ReadDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ParseIntError> for ReadDataError {
    fn from(err: ParseIntError) -> Self {
        ReadDataError::ParseError(err)
    }
}

impl From<std::io::Error> for ReadDataError {
    fn from(err: std::io::Error) -> Self {
        ReadDataError::IoError(err)
    }
}

fn main() {
    let data = read_data_from_path(Some("data.txt".to_string()));
    match data {
        Ok(data) => println!(":-) {}", data),
        Err(ReadDataError::ParseError(err)) => eprintln!(":-( {}", err),
        Err(ReadDataError::IoError(_)) => eprintln!(":-( Problems with the file")
    }
}
