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

/* crate use */
use csv;

/* project use */
use io;
use type_def::WorkOnWichPart;
use generator;


pub struct Indexing {
    part_number: u32,
    index_type: WorkOnWichPart,
    file_index_path: String,
    index_table: HashMap<String, Vec<(u64, u64)>>,
}

impl Indexing {
    pub fn new(file_index_path: &str, index_type: &str) -> Self {
        Indexing {
            part_number: 0,
            file_index_path: file_index_path.to_string(),
            index_type: WorkOnWichPart::from(index_type),
            index_table: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Indexing {
            part_number: 0,
            file_index_path: "".to_string(),
            index_type: WorkOnWichPart::Both,
            index_table: HashMap::new(),
        }
    }

    fn insert_index(self: &mut Self, entry: String, position: (u64, u64)) {
        self.index_table.entry(entry).or_insert_with(|| Vec::with_capacity(1)).push(position);

        if self.index_table.len() > 50000 {
            self.part_number += 1;
            Indexing::write_index(self.index_table.clone(), format!("{}_{}", self.file_index_path, self.part_number));
            self.index_table = HashMap::new();
        }
    }

    fn write_index(index: HashMap<String, Vec<(u64, u64)>>, index_path: String) {        
        if index_path == "".to_string() {
            return ;
        }

        let mut writer = csv::Writer::from_path(&index_path).expect("Can't create file to write part of index");

        let mut key_order = index.keys().collect::<Vec<&String>>();
        key_order.sort();
        
        for key in key_order {
            let val = index.get(key).unwrap();
            
            let mut iterator = val.iter();
            let mut position = iterator.next().unwrap().clone();
            
            let mut positions: Vec<(u64, u64)> = Vec::new();
            for v in iterator {

                if v.0 - position.1 > 1 {
                    positions.push(position);
                    position = v.clone();
                }
                else {
                    position.1 = v.1;
                }
            }
            positions.push(position);

            let positions_str = positions.iter().map(|x| x.0.to_string() + ":" + &x.1.to_string()).collect::<Vec<String>>().join(";");
            writer.write_record(&[key, &positions_str]).expect("Error durring write index file");
        }
    }
    
    fn run_both(self: &mut Self, r: &mut io::MappingRecord) {
        if r.read_a() == r.read_b() {
            self.insert_index(r.read_a(), r.position());
        } else {
            self.insert_index(r.read_a(), r.position());
            self.insert_index(r.read_b(), r.position());
        }        
    }

    fn run_query(self: &mut Self, r: &mut io::MappingRecord) {
        self.insert_index(r.read_a(), r.position());
    }

    fn run_target(self: &mut Self, r: &mut io::MappingRecord) {
        self.insert_index(r.read_b(), r.position());
    }

    fn merge(path_file1: &String, path_file2: &String) -> String {
        let new_filename = path_file1.to_string() + path_file2;

        let mut file1 = csv::Reader::from_path(&path_file1).expect("Open part of index");
        let mut file2 = csv::Reader::from_path(&path_file1).expect("Open part of index");
        let writer = csv::Writer::from_path(&new_filename).expect("Can't create file to store part of index");

        let mut records1 = file1.records();
        let mut records2 = file2.records();

        let mut record1 = records1.next();
        let mut record2 = records2.next();

        while record1.is_some() && record2.is_some() {
            let r1 = record1.unwrap().unwrap();
            let r2 = record2.unwrap().unwrap();

            if r1[0] == r2[0] {
                //write_twice

                record1 = records1.next();
                record2 = records2.next();
            } else if r1[0] < r2[0] {
                // write r1
                record1 = records1.next();
            } else {
                // write r2
                record2 = records2.next();
            }
        }
        
        return new_filename
    }
}

impl generator::Modifier for Indexing {
    fn run(self: &mut Self, r: &mut io::MappingRecord) {
        if self.file_index_path == "".to_string() {
            return ;
        }
        
        match self.index_type {
            WorkOnWichPart::Both => self.run_both(r),
            WorkOnWichPart::Query => self.run_query(r),
            WorkOnWichPart::Target => self.run_target(r),
        }
    }
    
    fn write(self: &mut Self) {
        std::fs::File::create(&self.file_index_path).expect("Can't create file to write index");

        let mut path = std::path::PathBuf::new();
        path.push(&self.file_index_path);
        path = std::fs::canonicalize(path).unwrap();

        let file_prefix = path.file_name().unwrap().to_str().unwrap().to_string();

        path.pop();
        
        let mut queue_file: Vec<String> = Vec::new();
        for entry in path.read_dir().expect("Can't loop on dir") {
            if let Ok(entry) = entry {
                let entry_filename = entry.path().file_name().unwrap().to_str().unwrap().to_string();
                if entry_filename != file_prefix && entry_filename.starts_with(&file_prefix) {
                    queue_file.push(entry_filename);
                }
            }
        }

        while queue_file.len() != 1 {
            let mut tmp_queue = Vec::new();
            for files in queue_file.chunks(2) {
                if files.len() == 2 {
                    tmp_queue.push(Indexing::merge(&files[0], &files[1]));
                } else {
                    tmp_queue.push(files[0].clone());
                }
            }
            queue_file = tmp_queue;
            println!("{:?}", queue_file);
        }
    }
}
