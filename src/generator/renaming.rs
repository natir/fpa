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

/* std use */
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/* project use */
use crate::generator;
use crate::io;

pub struct Renaming {
    file_rename_path: String,
    rename_table: HashMap<String, String>,
    index: u64,
    index_mode: bool,
}

impl Renaming {
    pub fn new(file_rename_path: &str, in_file: bool) -> Self {
        if in_file {
            if !Path::new(file_rename_path).exists() {
                panic!("Rename file not exist")
            }

            let mut table = HashMap::new();
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(File::open(file_rename_path).unwrap());

            for result in reader.records() {
                let record = result.expect("Error during parse of renaming file");
                table.insert(record[0].to_string(), record[1].to_string());
            }

            Renaming {
                file_rename_path: file_rename_path.to_string(),
                rename_table: table,
                index: 0,
                index_mode: false,
            }
        } else {
            Renaming {
                file_rename_path: file_rename_path.to_string(),
                rename_table: HashMap::new(),
                index: 1,
                index_mode: true,
            }
        }
    }

    fn run_index(self: &Self, r: &mut dyn io::MappingRecord) {
        if self.rename_table.contains_key(&r.read_a()) {
            let key = r.read_a();
            r.set_read_a(self.rename_table.get(&key).unwrap().to_string());
        }

        if self.rename_table.contains_key(&r.read_b()) {
            let key = r.read_b();
            r.set_read_b(self.rename_table.get(&key).unwrap().to_string());
        }
    }

    fn run_no_index(self: &mut Self, r: &mut dyn io::MappingRecord) {
        let mut key = r.read_a();
        if !self.rename_table.contains_key(&key) {
            self.rename_table.insert(r.read_a(), self.index.to_string());
            self.index += 1;
        }

        r.set_read_a(self.rename_table.get(&key).unwrap().to_string());

        key = r.read_b();
        if !self.rename_table.contains_key(&key) {
            self.rename_table.insert(r.read_b(), self.index.to_string());
            self.index += 1;
        }
        r.set_read_b(self.rename_table.get(&key).unwrap().to_string());
    }
}

impl generator::Modifier for Renaming {
    fn run(self: &mut Self, r: &mut dyn io::MappingRecord) {
        if self.index_mode {
            self.run_no_index(r);
        } else {
            self.run_index(r);
        }
    }

    fn write(self: &mut Self) {
        if self.index != 0 {
            let mut writer = csv::Writer::from_path(&self.file_rename_path)
                .expect("Can't create file to write renaming file");

            for (key, val) in &self.rename_table {
                writer
                    .write_record(&[key, val])
                    .expect("Error durring write renaming file");
            }
        }
    }
}
