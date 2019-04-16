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

use cli::Filters;

pub struct Keep {
    filters: Vec<Box<filter::Filter>>,
    internal_threshold: f64,
}

impl Keep {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let filters = Vec::new();
        let mut k = Keep {
            filters: filters,
            internal_threshold: matches.value_of("internal-match-threshold").unwrap().parse::<f64>().unwrap(),
        };

        if let Some(keep) = matches.subcommand_matches("keep") {
            k.generate(keep);
        }
        
        return k;
    }
}

impl Filters for Keep {
    fn pass(&self, r: &io::MappingRecord) -> bool {
        return if self.filters.is_empty() {
            return true
        } else {
            self.filters.iter().all(|ref x| x.run(r))
        };
    }

    fn internal_match(&self) -> f64 {
        self.internal_threshold
    }
    
    fn add_filter(&mut self, f: Box<filter::Filter>) {
        self.filters.push(f);
    }
}

