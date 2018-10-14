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

pub fn basic<R: std::io::Read, W: std::io::Write>(inputs: Vec<R>, output: &mut W, filters: &Vec<Box<filter::Filter>>, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>, format: work::InOutFormat) {
 
    if format == work::InOutFormat::Mhap {
        let mut writer = io::mhap::Writer::new(output);

        for input in inputs {
            mhap(io::mhap::Reader::new(input), &mut writer, &filters, modifiers);
        }
    } else {
        let mut writer = io::paf::Writer::new(output);

        for input in inputs {
            paf(io::paf::Reader::new(input), &mut writer, &filters, modifiers);
        }
    }
    
    for modifier in modifiers.iter_mut() {
        let m : &RefCell<modifier::Modifier> = &*modifier.clone();
        m.borrow_mut().write();
    }
}
    
fn paf<R: std::io::Read, W: std::io::Write>(mut reader: io::paf::Reader<R>, writer: &mut io::paf::Writer<W>, filters: &Vec<Box<filter::Filter>>, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>) {
    for result in reader.records() {
        let mut record = result.expect("Trouble during read of input");

        if filters.iter().any(|ref x| x.run(&record)) {
            continue;
        }
        
        for modifier in modifiers.iter_mut() {
            let m : &RefCell<modifier::Modifier> = &*modifier.clone();
            m.borrow_mut().run(&mut record);
        }

        writer.write(&record).expect("Trouble during write of output");
    }
}

fn mhap<'a, R: std::io::Read, W: std::io::Write>(mut reader: io::mhap::Reader<R>, writer: &mut io::mhap::Writer<W>, filters: &Vec<Box<filter::Filter>>, modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>) {
    for result in reader.records() {
        let mut record = result.expect("Trouble during read of input");

        if filters.iter().any(|ref x| x.run(&record)) {
            continue;
        }

        for modifier in modifiers.iter_mut() {
            let m : &RefCell<modifier::Modifier> = &*modifier.clone();
            m.borrow_mut().run(&mut record);
        }

        writer.write(&record).expect("Trouble during write of output");
    }
}


