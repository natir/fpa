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

pub mod gfa;
pub mod m4;
pub mod paf;

pub trait MappingRecord {
    fn read_a(&self) -> String;
    fn length_a(&self) -> u64;
    fn begin_a(&self) -> u64;
    fn end_a(&self) -> u64;
    fn strand(&self) -> char;
    fn read_b(&self) -> String;
    fn length_b(&self) -> u64;
    fn begin_b(&self) -> u64;
    fn end_b(&self) -> u64;
    fn position(&self) -> (u64, u64);
    fn set_position(&mut self, p: (u64, u64));

    fn length(&self) -> u64;

    fn len_to_end_a(&self) -> u64;
    fn len_to_end_b(&self) -> u64;

    fn set_read_a(&mut self, new_name: String);
    fn set_read_b(&mut self, new_name: String);
}

pub enum MappingFormat {
    Paf,
    M4,
}
