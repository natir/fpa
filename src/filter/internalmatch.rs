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
use std::cmp::{min, max};

pub struct InternalMatch {
    internal_threshold: f64,
    reverse: bool,
}

impl InternalMatch {
    pub fn new(reverse: bool) -> Self {
        InternalMatch {
            internal_threshold: 0.8,
            reverse: reverse,
        }
    }
}

impl filter::Filter for InternalMatch {
    
    fn run(self: &Self, r: &io::paf::Record) -> bool {
        let overhang = if r.strand == '+' {
            min(r.begin_a, r.begin_b) + min(r.length_a - r.end_a, r.length_b - r.end_b)
        } else {
            min(r.begin_a, r.length_b - r.end_b) + min(r.begin_b, r.length_a - r.end_a)
        };
        
        let maplen = max(r.end_a - r.begin_a, r.end_b - r.begin_b);

        let test = overhang > (maplen as f64 * self.internal_threshold) as u64;

        return if self.reverse { !test } else { test };
    }
}


