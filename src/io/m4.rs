/*
Copyright (c) 2018 Pierre Marijon <pierre.marijon@inria.fr>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

/* local use */
use crate::io;

/* standard use */
use std::cmp::min;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub read_a: String,
    pub read_b: String,
    pub error: f64,
    pub shared_min_mers: f64,
    pub strand_a: char,
    pub begin_a: u64,
    pub end_a: u64,
    pub length_a: u64,
    pub strand_b: char,
    pub begin_b: u64,
    pub end_b: u64,
    pub length_b: u64,
    pub position: (u64, u64),
}

impl io::MappingRecord for Record {
    fn read_a(self: &Self) -> String {
        self.read_a.clone()
    }

    fn length_a(self: &Self) -> u64 {
        self.length_a
    }

    fn begin_a(self: &Self) -> u64 {
        self.begin_a
    }

    fn end_a(self: &Self) -> u64 {
        self.end_a
    }

    fn strand(self: &Self) -> char {
        if self.strand_a == self.strand_b {
            '+'
        } else {
            '-'
        }
    }

    fn read_b(self: &Self) -> String {
        self.read_b.clone()
    }

    fn length_b(self: &Self) -> u64 {
        self.length_b
    }

    fn begin_b(self: &Self) -> u64 {
        self.begin_b
    }

    fn end_b(self: &Self) -> u64 {
        self.end_b
    }

    fn position(self: &Self) -> (u64, u64) {
        self.position
    }

    fn set_position(self: &mut Self, p: (u64, u64)) {
        self.position = p;
    }

    fn length(self: &Self) -> u64 {
        min(self.end_a - self.begin_a, self.end_b - self.begin_b)
    }

    fn len_to_end_a(self: &Self) -> u64 {
        self.length_a - self.end_a
    }
    fn len_to_end_b(self: &Self) -> u64 {
        self.length_b - self.end_b
    }

    fn set_read_a(self: &mut Self, new_name: String) {
        self.read_a = new_name;
    }
    fn set_read_b(self: &mut Self, new_name: String) {
        self.read_b = new_name;
    }
}

type RecordInner = (
    String,
    String,
    f64,
    f64,
    char,
    u64,
    u64,
    u64,
    char,
    u64,
    u64,
    u64,
);

pub struct Records<'a, R: 'a + std::io::Read> {
    inner: csv::DeserializeRecordsIter<'a, R, RecordInner>,
}

impl<'a, R: std::io::Read> Iterator for Records<'a, R> {
    type Item = csv::Result<Record>;

    fn next(&mut self) -> Option<csv::Result<Record>> {
        let position = self.inner.reader().position().byte();
        self.inner.next().map(|res| {
            res.map(
                |(
                    read_a,
                    read_b,
                    error,
                    shared_min_mers,
                    strand_a,
                    begin_a,
                    end_a,
                    length_a,
                    strand_b,
                    begin_b,
                    end_b,
                    length_b,
                )| {
                    let new_position = self.inner.reader().position().byte();

                    Record {
                        read_a,
                        read_b,
                        error,
                        shared_min_mers,
                        strand_a,
                        begin_a,
                        end_a,
                        length_a,
                        strand_b,
                        begin_b,
                        end_b,
                        length_b,
                        position: (position, new_position),
                    }
                },
            )
        })
    }
}

pub struct Reader<R: std::io::Read> {
    inner: csv::Reader<R>,
}

impl<R: std::io::Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader {
            inner: csv::ReaderBuilder::new()
                .delimiter(b' ')
                .has_headers(false)
                .flexible(true)
                .from_reader(reader),
        }
    }

    /// Iterate over all records.
    pub fn records(&mut self) -> Records<R> {
        Records {
            inner: self.inner.deserialize(),
        }
    }
}

#[derive(Debug)]
pub struct Writer<W: std::io::Write> {
    inner: csv::Writer<W>,
}

impl Writer<fs::File> {
    /// Write to a given file path in given format.
    #[allow(dead_code)]
    pub fn to_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        fs::File::create(path).map(Writer::new)
    }
}

impl<W: std::io::Write> Writer<W> {
    /// Write to a given writer.
    pub fn new(writer: W) -> Self {
        Writer {
            inner: csv::WriterBuilder::new()
                .delimiter(b' ')
                .has_headers(false)
                .flexible(true)
                .from_writer(writer),
        }
    }

    /// Write a given Blasr m4 record.
    pub fn write(&mut self, record: &Record) -> csv::Result<u64> {
        let buffer: Vec<u8> = Vec::new();
        let mut wrapper = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .flexible(true)
            .from_writer(buffer);

        wrapper.serialize((
            &record.read_a,
            &record.read_b,
            record.error,
            record.shared_min_mers,
            record.strand_a,
            record.begin_a,
            record.end_a,
            record.length_a,
            record.strand_b,
            record.begin_b,
            record.end_b,
            record.length_b,
        ))?;

        let nb_bytes = wrapper.into_inner().unwrap().len() as u64;

        self.inner.serialize((
            &record.read_a,
            &record.read_b,
            record.error,
            record.shared_min_mers,
            record.strand_a,
            record.begin_a,
            record.end_a,
            record.length_a,
            record.strand_b,
            record.begin_b,
            record.end_b,
            record.length_b,
        ))?;

        Ok(nb_bytes)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const M4_FILE: &'static [u8] = b"1 2 0.1 2 0 100 450 1000 0 550 900 1000
1 3 0.1 2 0 550 900 1000 0 100 450 1000
";

    const READ_A: &'static [&str; 2] = &["1", "1"];
    const READ_B: &'static [&str; 2] = &["2", "3"];
    const ERROR: &'static [f64; 2] = &[0.1, 0.1];
    const SHARED_MIN_MERS: &'static [f64; 2] = &[2.0, 2.0];
    const STRAND_A: &'static [char; 2] = &['0', '0'];
    const STRAND_B: &'static [char; 2] = &['0', '0'];
    const BEGIN_A: &'static [u64; 2] = &[100, 550];
    const END_A: &'static [u64; 2] = &[450, 900];
    const LENGTH_A: &'static [u64; 2] = &[1000, 1000];
    const BEGIN_B: &'static [u64; 2] = &[550, 100];
    const END_B: &'static [u64; 2] = &[900, 450];
    const LENGTH_B: &'static [u64; 2] = &[1000, 1000];

    #[test]
    fn read() {
        let mut reader = Reader::new(M4_FILE);

        for (i, r) in reader.records().enumerate() {
            let record = r.unwrap();

            assert_eq!(record.read_a, READ_A[i]);
            assert_eq!(record.read_b, READ_B[i]);
            assert_eq!(record.error, ERROR[i]);
            assert_eq!(record.shared_min_mers, SHARED_MIN_MERS[i]);
            assert_eq!(record.strand_a, STRAND_A[i]);
            assert_eq!(record.begin_a, BEGIN_A[i]);
            assert_eq!(record.end_a, END_A[i]);
            assert_eq!(record.length_a, LENGTH_A[i]);
            assert_eq!(record.strand_b, STRAND_B[i]);
            assert_eq!(record.begin_b, BEGIN_B[i]);
            assert_eq!(record.end_b, END_B[i]);
            assert_eq!(record.length_b, LENGTH_B[i]);
        }
    }

    #[test]
    fn write() {
        let mut reader = Reader::new(M4_FILE);
        let mut writer = Writer::new(vec![]);
        for r in reader.records() {
            writer
                .write(&r.ok().expect("Error reading record"))
                .ok()
                .expect("Error writing record");
        }
        assert_eq!(writer.inner.into_inner().unwrap(), M4_FILE);
    }
}
