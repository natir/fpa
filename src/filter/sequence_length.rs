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

/* project use */
use crate::filter;
use crate::io;

pub struct SequenceLength {
    length_threshold: u64,
    ordering: std::cmp::Ordering,
}

impl SequenceLength {
    pub fn new(length_threshold: u64, ord: std::cmp::Ordering) -> Self {
        SequenceLength {
            length_threshold,
            ordering: ord,
        }
    }
}

impl filter::Filter for SequenceLength {
    fn run(&self, r: &dyn io::MappingRecord) -> bool {
        r.length_a().cmp(&self.length_threshold) == self.ordering
            || r.length_b().cmp(&self.length_threshold) == self.ordering
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use filter::Filter;

    lazy_static! {
        static ref RECORD: io::paf::Record = {
            io::paf::Record {
                read_a: "read_1".to_string(),
                length_a: 5000,
                begin_a: 0,
                end_a: 5000,
                strand: '+',
                read_b: "read_2".to_string(),
                length_b: 20000,
                begin_b: 5000,
                end_b: 10000,
                nb_match_base: 500,
                nb_base: 500,
                mapping_quality: 255,
                sam_field: Vec::new(),
                position: (0, 50),
            }
        };
    }

    #[test]
    fn positif() {
        let mut nm = SequenceLength::new(5001, std::cmp::Ordering::Less);

        assert_eq!(nm.run(&*RECORD), true);

        nm = SequenceLength::new(20001, std::cmp::Ordering::Greater);

        assert_eq!(nm.run(&*RECORD), false);
    }

    #[test]
    fn negatif() {
        let mut nm = SequenceLength::new(5001, std::cmp::Ordering::Less);

        assert_ne!(nm.run(&*RECORD), false);

        nm = SequenceLength::new(20001, std::cmp::Ordering::Greater);

        assert_ne!(nm.run(&*RECORD), true);
    }
}
