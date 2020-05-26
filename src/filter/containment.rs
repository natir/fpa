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

pub struct Containment {
    internal_threshold: f64,
}

impl Containment {
    pub fn new(internal_threshold: f64) -> Self {
        Containment { internal_threshold }
    }
}

impl filter::Filter for Containment {
    fn run(self: &Self, r: &dyn io::MappingRecord) -> bool {
	if filter::InternalMatch::new(self.internal_threshold).run(r) {
	    return false
	}

	(r.strand() == '+' && r.begin_a() <= r.begin_b() && r.length_a() - r.end_a() < r.length_b() - r.end_b()) ||
	    (r.strand() == '-' && r.begin_a() <= r.length_b() - r.end_b() && r.length_a() - r.end_a() < r.begin_b()) ||
	    (r.strand() == '+' && r.begin_a() >= r.begin_b() && r.length_a() - r.end_a() > r.length_b() - r.end_b()) ||
	    (r.strand() == '-' && r.begin_a() >= r.length_b() - r.end_b() && r.length_a() - r.end_a() > r.begin_b())
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
        let nm = Containment::new(0.8);

        assert_eq!(nm.run(&*RECORD), true);
    }

    #[test]
    fn negatif() {
        let nm = Containment::new(0.8);

        assert_ne!(nm.run(&*RECORD), false);
    }
}
