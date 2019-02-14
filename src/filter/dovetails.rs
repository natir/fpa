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
use io;
use filter;

/* standard use */

pub struct Dovetails {
    internal_threshold: f64,
    reverse: bool,
}

impl Dovetails {
    pub fn new(internal_threshold: f64, reverse: bool) -> Self {
        Dovetails {
            internal_threshold: internal_threshold,
            reverse: reverse,
        }
    }
}

impl filter::Filter for Dovetails {
    fn run(self: &Self, r: &io::MappingRecord) -> bool {
        let test = !filter::Containment::new(self.internal_threshold, false).run(r);

        return if self.reverse { !test } else { test };
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use filter::Filter;

    lazy_static! {
        static ref RECORD: io::paf::Record = {
            io::paf::Record {
                read_a          : "read_1".to_string(),
                length_a        : 20000,
                begin_a         : 15000,
                end_a           : 20000,
                strand          : '+',
                read_b          : "read_2".to_string(),
                length_b        : 20000,
                begin_b         : 0,
                end_b           : 15000,
                nb_match_base   : 500,
                nb_base         : 500,
                mapping_quality : 255,
                sam_field       : Vec::new(),
                position        : (0, 50),
            }
        }; 
    }

    #[test]
    fn positif() {
        let mut nm = Dovetails::new(0.8, false);

        assert_eq!(nm.run(&*RECORD), true);

        nm = Dovetails::new(0.8, true);

        assert_eq!(nm.run(&*RECORD), false);
    }

    #[test]
    fn negatif() {
        let mut nm = Dovetails::new(0.8, false);

        assert_ne!(nm.run(&*RECORD), false);

        nm = Dovetails::new(0.8, true);

        assert_ne!(nm.run(&*RECORD), true);
    }
}
