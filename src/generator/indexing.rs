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

/* project use */
use crate::generator;
use crate::io;
use crate::type_def::WorkOnWichPart;

pub struct Indexing {
    index_type: WorkOnWichPart,
    file_index_path: String,
    index_table: HashMap<String, Vec<(u64, u64)>>,
}

impl Indexing {
    pub fn new(file_index_path: &str, index_type: &str) -> Self {
        Indexing {
            file_index_path: file_index_path.to_string(),
            index_type: WorkOnWichPart::from(index_type),
            index_table: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Indexing {
            file_index_path: "".to_string(),
            index_type: WorkOnWichPart::Both,
            index_table: HashMap::new(),
        }
    }

    fn run_both(self: &mut Self, r: &mut dyn io::MappingRecord) {
        if r.read_a() == r.read_b() {
            self.index_table
                .entry(r.read_a())
                .or_insert_with(Vec::new)
                .push(r.position());
        } else {
            self.index_table
                .entry(r.read_a())
                .or_insert_with(Vec::new)
                .push(r.position());
            self.index_table
                .entry(r.read_b())
                .or_insert_with(Vec::new)
                .push(r.position());
        }
    }

    fn run_query(self: &mut Self, r: &mut dyn io::MappingRecord) {
        self.index_table
            .entry(r.read_a())
            .or_insert_with(Vec::new)
            .push(r.position());
    }

    fn run_target(self: &mut Self, r: &mut dyn io::MappingRecord) {
        self.index_table
            .entry(r.read_b())
            .or_insert_with(Vec::new)
            .push(r.position());
    }
}

impl generator::Modifier for Indexing {
    fn run(self: &mut Self, r: &mut dyn io::MappingRecord) {
        if self.file_index_path == "" {
            return;
        }

        match self.index_type {
            WorkOnWichPart::Both => self.run_both(r),
            WorkOnWichPart::Query => self.run_query(r),
            WorkOnWichPart::Target => self.run_target(r),
        }
    }

    fn write(self: &mut Self) {
        if self.file_index_path == "" {
            return;
        }

        let mut writer = csv::Writer::from_path(&self.file_index_path)
            .expect("Can't create file to write index");

        for (key, val) in &self.index_table {
            let mut iterator = val.iter();
            let mut position = *iterator.next().unwrap();

            let mut positions: Vec<(u64, u64)> = Vec::new();
            for v in iterator {
                if v.0 - position.1 > 1 {
                    positions.push(position);
                    position = *v;
                } else {
                    position.1 = v.1;
                }
            }
            positions.push(position);

            let positions_str = positions
                .iter()
                .map(|x| x.0.to_string() + ":" + &x.1.to_string())
                .collect::<Vec<String>>()
                .join(";");
            writer
                .write_record(&[key, &positions_str])
                .expect("Error durring write index file");
        }
    }
}
