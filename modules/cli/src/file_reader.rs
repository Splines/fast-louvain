// adapted from https://stackoverflow.com/a/45882510/9655481
use std::{
    fs::File,
    io::{self, prelude::*},
};

pub struct BufReader {
    reader: io::BufReader<File>,
}

impl BufReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        Ok(Self { reader })
    }

    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> Option<io::Result<&'buf mut String>> {
        buffer.clear();

        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }

    /// Returns the number of lines in the file.
    pub fn num_non_empty_lines(&mut self) -> usize {
        let mut count = 0;
        let mut buffer: &[u8];
        loop {
            buffer = self.reader.fill_buf().unwrap();
            if buffer.is_empty() {
                break;
            }
            count += bytecount::count(&buffer, b'\n');
            let len = buffer.len();
            self.reader.consume(len);
        }

        // Rewind to beginning of file
        self.reader.seek(io::SeekFrom::Start(0)).unwrap();

        count
    }
}
