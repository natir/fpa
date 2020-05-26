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
use crate::generator;
use crate::io;

pub struct Modifier {
    modifiers: Vec<Box<dyn generator::Modifier>>,
}

impl Modifier {
    pub fn new(
        internal_match: f64,
        matches: &std::collections::HashMap<String, clap::ArgMatches>,
    ) -> Self {
        let mut modifiers: Vec<Box<dyn generator::Modifier>> = Vec::new();

        if let Some(m) = matches.get("rename") {
            if m.is_present("input") {
                modifiers.push(Box::new(generator::Renaming::new(
                    m.value_of("input").unwrap(),
                    true,
                )));
            } else if m.is_present("output") {
                modifiers.push(Box::new(generator::Renaming::new(
                    m.value_of("output").unwrap(),
                    false,
                )));
            }
        }

        if let Some(m) = matches.get("gfa") {
            modifiers.push(Box::new(generator::Gfa1::new(
                m.value_of("output").unwrap().to_string(),
                m.is_present("internalmatch"),
                m.is_present("containment"),
                internal_match,
            )))
        }

        Modifier { modifiers }
    }

    pub fn pass(&mut self, r: &mut dyn io::MappingRecord) {
        for m in self.modifiers.iter_mut() {
            m.run(r);
        }
    }

    pub fn write(&mut self) {
        for m in self.modifiers.iter_mut() {
            m.write();
        }
    }
}
