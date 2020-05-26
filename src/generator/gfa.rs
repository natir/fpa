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

/* crate use */

/* projet use */
use crate::generator;
use crate::io;

pub struct Gfa1 {
    gfa_path: String,
    gfa_object: io::gfa::Gfa1,
}

impl Gfa1 {
    pub fn new(
        gfa_path: String,
        keep_internal: bool,
        keep_containment: bool,
        internal_threshold: f64,
    ) -> Self {
        Gfa1 {
            gfa_path,
            gfa_object: io::gfa::Gfa1::new(keep_internal, keep_containment, internal_threshold),
        }
    }
}

impl generator::Modifier for Gfa1 {
    fn run(self: &mut Self, r: &mut dyn io::MappingRecord) {
        self.gfa_object.add(r);
    }

    fn write(self: &mut Self) {
        let mut writer = std::fs::File::create(&self.gfa_path).unwrap();
        self.gfa_object.write(&mut writer);
    }
}
