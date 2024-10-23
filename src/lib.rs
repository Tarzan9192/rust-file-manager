use std::fmt::Write as FmtWrite;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Error, Write},
    path::Path,
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
/// If `truncate == true`, the file will be truncated before writing `contents`.
fn open_file_for_writing(file_path: &str, truncate: bool) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .truncate(truncate)
        .create(true)
        .open(file_path)
}

/// Helper function to open a file with append privelages.
/// It will create the file if it does not already exist at `file_path`.
fn open_file_for_appending(file_path: &str) -> io::Result<File> {
    OpenOptions::new().append(true).create(true).open(file_path)
}

/// Opens a file at `file_path` for appending.
/// If the file does not exist, it will be created at `file_path`.
/// Contents written to the returned `BufWriter<File>` will be appended to the end of the file.
///
/// # Returns
/// A `BufWriter` for writing contents to the file.
pub fn open_buffered_file_appender(file_path: &str) -> Result<BufWriter<File>, Error> {
    let file = open_file_for_appending(file_path)?;

    Ok(BufWriter::new(file))
}

/// Attempts to append `contents` to file at `file_path`.
/// Will create file at `file_path` if it does not already exist.
/// Each call to this funciton will append a platform specific newline character.
/// If `truncate == true`, the file will be truncated before writing `contents`.
pub fn append_to_file(file_path: &str, contents: &str) -> Result<(), io::Error> {
    let mut file = open_file_for_appending(file_path)?;

    // Hacky way to get env specific newline char after each function call.
    let mut s = String::new();
    let _ = writeln!(&mut s, "{}", contents);

    // Write the string with newline char appended.
    file.write_all(s.as_bytes())?;

    // Make sure all bytes have been written.
    file.flush()?;
    Ok(())
}

/// Opens a file at `file_path` for writing.
/// If the file does not exist, it will be created at `file_path`.
/// If `truncate == true`, the file will be truncated before writing `contents`.
///
/// # Returns
/// A `BufWriter` for writing contents to the file.
pub fn open_buffered_file_writer(
    file_path: &str,
    truncate: bool,
) -> Result<BufWriter<File>, Error> {
    let file = open_file_for_writing(file_path, truncate)?;

    Ok(BufWriter::new(file))
}

/// Attempts to write `contents` to file at `file_path`.
/// Will create file at `file_path` if it does not already exist.
/// If `truncate == true`, the file will be truncated before writing `contents`.
pub fn write_to_file(file_path: &str, truncate: bool, contents: &str) -> Result<(), io::Error> {
    let mut file = open_file_for_writing(file_path, truncate)?;
    file.write_all(contents.as_bytes())?;

    // Make sure all bytes have been written.
    file.flush()?;
    Ok(())
}

/// This function creates an empty file at `file_path`.
/// This will truncate an existing file at `file_path` if `truncate == true`.
pub fn create_file(file_path: &str, truncate: bool) -> io::Result<()> {
    if Path::new(file_path).exists() && !truncate {
        // If the file exists and we do not want to truncate, do nothing.
        Ok(())
    } else {
        // Otherwise, just create the file. It will be truncated if it already exists.
        File::create(file_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::{BufRead, Read};
    use std::path::Path;

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
        let result = open_buffered_file_writer(file_path, false);

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
        let result = write_to_file(file_path, true, content);
        let mut parsed_content = String::new();
        if let Some(mut file) = open_file(file_path) {
            let _ = file.read_to_string(&mut parsed_content);
        }

        // assert
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists());
        assert_eq!(content, parsed_content.as_str());
    }

    #[test]
    // Make sure the file will not get truncated if create_file() \
    // is called on a path that already exists.
    fn create_file_no_truncate_works() {
        // arrange
        let file_path = "assets/create_file_no_truncate_test.txt";
        let mut before_test = String::new();
        let mut after_test = String::new();
        let _ = open_file(file_path)
            .unwrap()
            .read_to_string(&mut before_test);

        // act
        let result = create_file(file_path, false);
        let _ = open_file(file_path)
            .unwrap()
            .read_to_string(&mut after_test);

        // assert
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists());
        assert_eq!(before_test.trim(), after_test.trim());
    }

    #[test]
    // Make sure the file is truncated if `create_file` is called with `truncate == true`.
    fn create_file_truncate_works() {
        // arrange
        let file_path = "assets/create_file_truncate_test.txt";
        let contents = "some content";
        let mut after_test = String::new();
        let mut before_test = String::new();
        let _ = write_to_file(file_path, true, contents);
        let _ = open_file(file_path)
            .unwrap()
            .read_to_string(&mut before_test);

        // act
        let result = create_file(file_path, true);
        let _ = open_file(file_path)
            .unwrap()
            .read_to_string(&mut after_test);

        // assert
        assert!(result.is_ok());
        assert!(Path::new(file_path).exists());
        assert_eq!(contents.trim(), before_test.trim());
        assert!(after_test.is_empty());
    }

    #[test]
    fn append_to_file_works() {
        // arrange
        let file_path = "assets/append_file_test.txt";
        let first_line = "line 1";
        let second_line = "line 2";

        // Start off clean
        let _ = create_file(file_path, true);

        // act
        let _ = append_to_file(file_path, first_line);
        let result = append_to_file(file_path, second_line);
        let file = open_file(file_path).unwrap();
        let mut lines = file.lines().map(|l| l.unwrap());

        // assert
        assert!(result.is_ok());
        assert_eq!(Some(first_line.to_owned()), lines.next());
        assert_eq!(Some(second_line.to_owned()), lines.next());
    }
}
