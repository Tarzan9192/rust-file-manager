use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Error, ErrorKind, Write},
};

/// Attempt to open the file at `file_path` and return a BufReader<File>.
pub fn open_file(file_path: &str) -> Option<BufReader<File>> {
    // Open the file and read contents
    // Keep trying until we successfully open a file
    if let Ok(file) = File::open(file_path) {
        // Create a BufReader from the File
        Some(BufReader::new(file))
    } else {
        None
    }
}

/// Helper function to open a file with write privelages.
/// It will create the file if it does not already exist at `file_path`.
fn open_file_for_writing(file_path: &str) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).open(file_path)
}

/// Opens a file at `file_path` for writing.
/// If the file does not exist, it will be created at `file_path`.
///
/// # Returns
/// A `BufWriter` for writing contents to the file.
pub fn open_buffered_file_writer(file_path: &str) -> Result<BufWriter<File>, Error> {
    let file = open_file_for_writing(file_path)?;

    Ok(BufWriter::new(file))
}

/// Attempts to write `contents` to file at `file_path`.
/// Will create file at `file_path` if it does not already exist.
pub fn write_to_file(file_path: &str, contents: &str) -> Result<(), io::Error> {
    let mut file = open_file_for_writing(file_path)?;
    file.write_all(contents.as_bytes())?;

    // Make sure all bytes have been written.
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fmt::write, path::Path};

    use io::Read;

    use super::*;

    #[test]
    fn open_file_works() {
        // arrange
        let file_path = "assets/test.json";

        // act
        let result = open_file(file_path);

        // assert
        assert!(result.is_some())
    }

    #[test]
    fn open_file_works_if_file_not_exists() {
        // arrange
        let file_path = "test.json";

        // act
        let result = open_file(file_path);

        // assert
        assert!(result.is_none())
    }


    #[test]
    fn open_buffered_file_writer_works() {
        // arrange
        let file_path = "assets/buffered_test.json";

        // act
        let result = open_buffered_file_writer(file_path);

        // assert
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists())
    }


    #[test]
    fn write_to_file_works() {
        // arrange
        let file_path = "assets/write_to_file_test.txt";
        let content = "here is some content";

        // act
        let result = write_to_file(file_path, content);
        let mut parsed_content = String::new();
        if let Some(mut file) = open_file(file_path) {
            let _ = file.read_to_string(&mut parsed_content);
        }

        // assert
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists());
        assert_eq!(content, parsed_content.as_str());
    }
}
