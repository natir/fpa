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
use cli;
use work;
use filter;
use generator;

/* std use */
use std;
use std::rc::Rc;
use std::cell::RefCell;

/* crate use */
use clap::ArgMatches;

pub fn gfa1<R: std::io::Read, W: std::io::Write>(
    inputs: Vec<R>,
    output: &mut W,
    modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>,
    matches: &ArgMatches,
    format: work::InOutFormat,
) {

    let internal_match_t = matches
        .value_of("internal-match-threshold")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let keep_internalmatch = !matches.is_present("delete_internalmatch");
    let keep_containment = !matches.is_present("delete_containment");

    let mut gfa = io::gfa::Gfa1::new(keep_internalmatch, keep_containment, internal_match_t);

    if format == work::InOutFormat::Mhap {
        mhap(inputs, output, modifiers, matches, &mut gfa);
    } else {
        paf(inputs, output, modifiers, matches, &mut gfa);
    }
}

fn paf<'a, R: std::io::Read, W: std::io::Write>(
    inputs: Vec<R>,
    writer: &mut W,
    modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>,
    matches: &ArgMatches,
    gfa: &mut io::gfa::Gfa1,
) {

    let mut filters: Vec<Box<filter::Filter>> = Vec::new();
    cli::generate_other_filters(matches, &mut filters);

    for input in inputs {
        let mut reader = io::paf::Reader::new(input);
        for result in reader.records() {
            let mut r = result.expect("Error");

            if filters.iter().any(|ref x| x.run(&r)) {
                continue;
            }

            for modifier in modifiers.iter_mut() {
                let m: &RefCell<modifier::Modifier> = &*modifier.clone();
                m.borrow_mut().run(&mut r);
            }

            gfa.add(Box::new(r));
        }
    }

    gfa.write(writer);

    for modifier in modifiers.iter_mut() {
        let m: &RefCell<modifier::Modifier> = &*modifier.clone();
        m.borrow_mut().write();
    }
}


fn mhap<R: std::io::Read, W: std::io::Write>(
    inputs: Vec<R>,
    writer: &mut W,
    modifiers: &mut Vec<Rc<RefCell<modifier::Modifier>>>,
    matches: &ArgMatches,
    gfa: &mut io::gfa::Gfa1,
) {
    let mut filters: Vec<Box<filter::Filter>> = Vec::new();
    cli::generate_type_filters(matches, &mut filters);


    for input in inputs {
        let mut reader = io::mhap::Reader::new(input);
        for result in reader.records() {
            let mut r = result.expect("Error");

            if filters.iter().any(|ref x| x.run(&r)) {
                continue;
            }

            for modifier in modifiers.iter_mut() {
                let m: &RefCell<modifier::Modifier> = &*modifier.clone();
                m.borrow_mut().run(&mut r);
            }

            gfa.add(Box::new(r));
        }
    }

    gfa.write(writer);

    for modifier in modifiers.iter_mut() {
        let m: &RefCell<modifier::Modifier> = &*modifier.clone();
        m.borrow_mut().write();
    }
}


#[cfg(test)]
mod test {

    use super::*;

    use std::collections::HashSet;

    fn run_test(paf: &'static [u8], thrut: &'static [u8], mut gfa: io::gfa::Gfa1) {
        let mut reader = io::paf::Reader::new(paf);
        let mut writer = vec![];

        for result in reader.records() {
            let mut r = result.expect("Error");

            gfa.add(Box::new(r));
        }

        gfa.write(&mut writer);

        {
            // Debug test
            println!("Result\n{}", String::from_utf8_lossy(&writer));
            println!("True\n{}", String::from_utf8_lossy(thrut));
        }

        let thrut_str = String::from_utf8_lossy(thrut);
        let output_str = String::from_utf8_lossy(&writer);

        let thrut = thrut_str.split("\n").collect::<HashSet<&str>>();
        let output = output_str.split("\n").collect::<HashSet<&str>>();

        assert_eq!(thrut, output);

    }

    #[test]
    fn basic() {
        const PAF: &'static [u8] = b"1\t10000\t1999\t10000\t-\t2\t10000\t1999\t10000\t8001\t8001\t255
1\t10000\t0\t8000\t-\t5\t10000\t1\t8000\t7999\t7999\t255
2\t10000\t1\t8000\t-\t3	10000\t0\t8000\t8000\t8000\t255
3\t10000\t2000\t10000\t+\t4\t10000\t0\t8000\t8000\t8000\t255
";

        const GFA: &'static [u8] = b"H	VN:Z:1.0
S\t1\t*\tLN:i:10000
S\t2\t*\tLN:i:10000
S\t5\t*\tLN:i:10000
S\t3\t*\tLN:i:10000
S\t4\t*\tLN:i:10000
L\t1\t+\t2\t-\t8001M
L\t1\t-\t5\t+\t7999M
L\t2\t-\t3\t+\t7999M
L\t3\t+\t4\t+\t8000M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(false, false, 0.8));
    }

    #[test]
    fn two_repeat_strand_diff() {
        const PAF: &'static [u8] = b"1\t10000\t2000\t10000\t-\t2\t10000\t1999\t10000\t8001\t8001\t255
5\t10000\t1\t8000\t-\t1\t10000\t0\t8000\t8000\t8000\t255
2\t10000\t1\t8000\t-\t3\t10000\t0\t8000\t8000\t8000\t255
3\t10000\t2000\t10000\t+\t4\t10000\t0\t8000\t8000\t8000\t255
4\t10000\t2000\t10000\t+\t9\t10000\t0\t8000\t8000\t8000\t255
4\t10000\t2001\t10000\t-\t7\t10000\t2000\t10000\t8000\t8000\t255
8\t10000\t2000\t10000\t+\t7\t10000\t0\t8000\t8000\t8000\t255
7\t10000\t2000\t10000\t+\t6\t10000\t0\t8000\t8000\t8000\t255
6\t10000\t2001\t10000\t-\t5\t10000\t2000\t10000\t8000\t8000\t255
";

        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\t1\t*\tLN:i:10000
S\t2\t*\tLN:i:10000
S\t5\t*\tLN:i:10000
S\t3\t*\tLN:i:10000
S\t4\t*\tLN:i:10000
S\t9\t*\tLN:i:10000
S\t7\t*\tLN:i:10000
S\t8\t*\tLN:i:10000
S\t6\t*\tLN:i:10000
L\t1\t+\t2\t-\t8000M
L\t5\t-\t1\t+\t7999M
L\t2\t-\t3\t+\t7999M
L\t3\t+\t4\t+\t8000M
L\t4\t+\t9\t+\t8000M
L\t4\t+\t7\t-\t7999M
L\t8\t+\t7\t+\t8000M
L\t7\t+\t6\t+\t8000M
L\t6\t+\t5\t-\t7999M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(false, false, 0.8));
    }

    // B is contain in A
    // --------------------------->
    //        --------->
    #[test]
    fn a_contain_b_keep_contain_strand_same() {
        const PAF: &'static [u8] = b"1\t2000\t500\t1500\t+\t2\t1000\t0\t1000\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\t1\t*\tLN:i:2000
S\t2\t*\tLN:i:1000
C\t1\t+\t2\t+\t500\t1000M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // B is contain in A
    // --------------------------->
    //        <---------
    #[test]
    fn a_contain_b_keep_contain_strand_diff() {
        const PAF: &'static [u8] = b"1\t2000\t500\t1500\t-\t2\t1000\t0\t1000\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\t1\t*\tLN:i:2000
S\t2\t*\tLN:i:1000
C\t1\t+\t2\t-\t500\t1000M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // A is contain in B
    //        --------->
    // --------------------------->
    #[test]
    fn b_contain_a_keep_contain_strand_same() {
        const PAF: &'static [u8] = b"2\t1000\t0\t1000\t+\t1\t2000\t500\t1500\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\t1\t*\tLN:i:2000
S\t2\t*\tLN:i:1000
C\t1\t+\t2\t+\t500\t1000M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // A is contain in B
    //      --------->
    // <---------------------------
    #[test]
    fn b_contain_a_keep_contain_strand_diff() {
        const PAF: &'static [u8] = b"2\t1000\t0\t1000\t-\t1\t2000\t500\t1500\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\t1\t*\tLN:i:2000
S\t2\t*\tLN:i:1000
C\t1\t+\t2\t-\t500\t1000M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // B is contain in A
    // --------------------------->
    //        --------->
    #[test]
    fn a_contain_b_leave_contain_strand_same() {
        const PAF: &'static [u8] = b"1\t2000\t500\t1500\t+\t2\t1000\t0\t1000\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0\n";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, false, 0.8));
    }

    // B is contain in A
    // --------------------------->
    //        <---------
    #[test]
    fn a_contain_b_leave_contain_strand_diff() {
        const PAF: &'static [u8] = b"1\t2000\t500\t1500\t-\t2\t1000\t0\t1000\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0\n";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, false, 0.8));
    }

    // A is contain in B
    //        --------->
    // --------------------------->
    #[test]
    fn b_contain_a_leave_contain_strand_same() {
        const PAF: &'static [u8] = b"2\t1000\t0\t1000\t+\t1\t2000\t500\t1500\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0\n";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, false, 0.8));
    }

    // A is contain in B
    //        --------->
    // <---------------------------
    #[test]
    fn b_contain_a_leave_contain_strand_diff() {
        const PAF: &'static [u8] = b"2\t1000\t0\t1000\t-\t1\t2000\t500\t1500\t30\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0\n";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, false, 0.8));
    }

    #[test]
    fn a_contai_b_b_contain_a() {
        const PAF: &'static [u8] = b"A\t1000\t0\t1000\t+\tB\t1000\t1\t999\t0\t1000\t255
B\t1000\t0\t1000\t+\tA\t1000\t1\t999\t0\t1000\t255";
        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\tA\t*\tLN:i:1000
S\tB\t*\tLN:i:1000
C\tB\t+\tA\t+\t1\t998M
C\tA\t+\tB\t+\t1\t998M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, true, 0.8));
    }

    #[test]
    fn duplication() {
        const PAF: &'static [u8] = b"A\t1000\t200\t1000\t+\tB\t1000\t0\t800\t800\t800\t255
A\t1000\t200\t1000\t+\tB\t1000\t0\t800\t800\t800\t255
B\t1000\t0\t800\t+\tA\t1000\t200\t1000\t800\t800\t255
";

        const GFA: &'static [u8] = b"H\tVN:Z:1.0
S\tA\t*\tLN:i:1000
S\tB\t*\tLN:i:1000
L\tA\t+\tB\t+\t800M
";

        run_test(PAF, GFA, io::gfa::Gfa1::new(true, false, 0.8));
    }

    // A and B internal match same strand
    //  --         -->
    //    \       /
    //     -------
    //     -------
    //    /       \
    //  --         -->
    #[test]
    fn a_b_internal_match_keep_same() {

        let paf = b"A\t10000\t200\t5100\t+\tB\t10000\t100\t5000\t30\t4900\t255";
        let resu = b"
H\tVN:Z:1.0
S\tA\t*\tLN:i:10000
S\tB\t*\tLN:i:10000
L\tA\t+\tB\t+\t4900M
";

        run_test(paf, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // A and B internal match diff strand
    //  --         -->
    //    \       /
    //     -------
    //     -------
    //    /       \
    // <--         --
    #[test]
    fn a_b_internal_match_keep_diff() {
        let line = b"A\t10000\t2200\t7100\t-\tB\t10000\t2100\t7000\t30\t4900\t255";
        let resu = b"
H\tVN:Z:1.0
S\tA\t*\tLN:i:10000
S\tB\t*\tLN:i:10000
L\tA\t-\tB\t+\t4900M
";
        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // A and B internal match same strand
    //  --         -->
    //    \       /
    //     -------
    //     -------
    //    /       \
    //  --         -->
    #[test]
    fn a_b_internal_match_remove_same() {
        let line = b"A\t10000\t200\t5100\t+\tB\t10000\t100\t5000\t30\t4900\t255";

        let resu = b"
H\tVN:Z:1.0
";
        run_test(line, resu, io::gfa::Gfa1::new(false, true, 0.8));
    }

    // A and B internal match diff strand
    //  --         -->
    //    \       /
    //     -------
    //     -------
    //    /       \
    // <--         --
    #[test]
    fn a_b_internal_match_remove_diff() {
        let line = b"A\t10000\t2200\t7100\t-\tB\t10000\t2100\t7000\t30\t4900\t255";

        let resu = b"
H\tVN:Z:1.0
";

        run_test(line, resu, io::gfa::Gfa1::new(false, true, 0.8));
    }

    #[test]
    fn test_minimap_3contain() {
        let line = b"56001	3693	79	3329	+	63107	23535	5870	9145	350	3275	255	cm:i:38
59727	27255	14393	26854	-	63107	23535	11119	23497	1298	12461	255	cm:i:145
59847	18269	51	18268	-	63107	23535	4047	22282	2124	18235	255	cm:i:247";

        let resu = b"
H	VN:Z:1.0
S	63107	*	LN:i:23535
S	56001	*	LN:i:3693
S	59727	*	LN:i:27255
S	59847	*	LN:i:18269
L	59727	+	63107	-	12378M
C	63107	+	56001	+	5870	3250M
C	63107	+	59847	-	4047	18217M
";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // overlap at A 3' same orientation
    // ---------------->
    //              ------------->
    #[test]
    fn test_a_3_same() {
        let line = b"1\t1000\t20\t1000\t+\t2\t1000\t0\t980\t30\t980\t255";
        let resu = b"H\tVN:Z:1.0\nS\t1\t*\tLN:i:1000\nS\t2\t*\tLN:i:1000\nL\t1\t+\t2\t+\t980M\n";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // overlap at A 3' orientation is different
    // ---------------->
    //             <-------------
    #[test]
    fn test_a_3_diff() {
        let line = b"1\t1000\t10\t1000\t-\t2\t1000\t10\t1000\t30\t980\t255";
        let resu = b"H\tVN:Z:1.0\nS\t1\t*\tLN:i:1000\nS\t2\t*\tLN:i:1000\nL\t1\t+\t2\t-\t990M\n";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // overlap at A 5' orientation is same
    //             ------------->
    // ---------------->
    #[test]
    fn test_a_5_same() {
        let line = b"1\t1000\t0\t980\t+\t2\t1000\t20\t1000\t30\t980\t255";
        let resu = b"H\tVN:Z:1.0\nS\t1\t*\tLN:i:1000\nS\t2\t*\tLN:i:1000\nL\t2\t+\t1\t+\t980M\n";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    // overlap at A 5' orientation is different
    //             ------------->
    // <----------------
    #[test]
    fn test_a_5_diff() {
        let line = b"1\t1000\t0\t980\t-\t2\t1000\t0\t980\t30\t960\t255";
        let resu = b"H\tVN:Z:1.0\nS\t1\t*\tLN:i:1000\nS\t2\t*\tLN:i:1000\nL\t1\t-\t2\t+\t980M\n";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    #[test]
    fn test_a_overlap_b_b_contained_c_keep_all() {

        let line = b"
B	8853	7897	8500	-	A	25804	24891	25773	150	882	255	cm:i:16
B	8853	5997	8553	-	C	2962	80	2901	511	2821	255	cm:i:76
";

        let resu = b"
H	VN:Z:1.0
S	B	*	LN:i:8853
S	A	*	LN:i:25804
S	C	*	LN:i:2962
L	B	+	A	-	603M
C	B	+	C	-	5997	2556M
";

        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }

    #[test]
    fn test_a_overlap_b_b_contained_c_leave_all() {

        let line = b"
B	8853	7897	8500	-	A	25804	24891	25773	150	882	255	cm:i:16
B	8853	5997	8553	-	C	2962	80	2901	511	2821	255	cm:i:76
";

        let resu = b"
H	VN:Z:1.0
S	B	*	LN:i:8853
S	A	*	LN:i:25804
L	B	+	A	-	603M
";
        run_test(line, resu, io::gfa::Gfa1::new(false, false, 0.8));
    }

    #[test]
    fn test_a_overlap_a() {

        let line = b"A\t17472\t15493\t17472\t-\tA\t17472\t15493\t17472\t314\t197\t255\tcm:i:38\n";

        let resu = b"H\tVN:Z:1.0
S\tA\t*\tLN:i:17472
L\tA\t+\tA\t-\t1979M
";
        run_test(line, resu, io::gfa::Gfa1::new(true, true, 0.8));
    }
}
