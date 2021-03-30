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

/* standard use */

pub struct NameMatch {
    regex: regex::Regex,
}

impl NameMatch {
    pub fn new(regex: &str) -> Self {
        NameMatch {
            regex: regex::Regex::new(regex).expect("Error in regex build"),
        }
    }
}

impl filter::Filter for NameMatch {
    fn run(&self, r: &dyn io::MappingRecord) -> bool {
        self.regex.is_match(&r.read_a()) || self.regex.is_match(&r.read_b())
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
                length_a: 20000,
                begin_a: 1,
                end_a: 19999,
                strand: '+',
                read_b: "read_2".to_string(),
                length_b: 20000,
                begin_b: 1,
                end_b: 19999,
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
        let nm = NameMatch::new("read_1");

        assert_eq!(nm.run(&*RECORD), true);
    }

    #[test]
    fn negatif() {
        let nm = NameMatch::new("read_1");

        assert_ne!(nm.run(&*RECORD), false);
    }
}
