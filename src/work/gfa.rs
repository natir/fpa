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
use work;
use filter;
use modifier;

/* std use */
use std;
use std::rc::Rc;
use std::cell::RefCell;

/* crate use */
use clap::ArgMatches;

struct Gfa1Node {
    readName: String,
    length: u64,
}

struct Gfa1 {
    keep_internal: bool,
    keep_containment: bool,
}

pub fn gfa1<R: std::io::Read, W: std::io::Write>(inputs: Vec<R>, output: &mut W, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>, matches: &ArgMatches, format: work::InOutFormat) {
    if format == work::InOutFormat::Mhap {
        //mhap();
    } else {
        //paf();
    }
}

fn paf<R: std::io::Read, W: std::io::Write>(mut reader: io::paf::Reader<R>, writer: &mut io::paf::Writer<W>, filters: &Vec<Box<filter::Filter>>, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>, matches: &ArgMatches) {

}


fn mhap<R: std::io::Read, W: std::io::Write>(mut reader: io::mhap::Reader<R>, writer: &mut io::mhap::Writer<W>, filters: &Vec<Box<filter::Filter>>, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>, matches: &ArgMatches) {

}
