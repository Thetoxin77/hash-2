#![allow(non_snake_case)]
#![feature(test)]

extern crate hash;
extern crate test;

use hash::md5;

#[bench] fn compute_____1_000(bencher: &mut test::Bencher) { compute(    1_000, bencher); }
#[bench] fn compute____10_000(bencher: &mut test::Bencher) { compute(   10_000, bencher); }
#[bench] fn compute___100_000(bencher: &mut test::Bencher) { compute(  100_000, bencher); }
#[bench] fn compute_1_000_000(bencher: &mut test::Bencher) { compute(1_000_000, bencher); }

fn compute(size: usize, bencher: &mut test::Bencher) {
    let input = &vec![0xFFu8; size][..];
    bencher.iter(|| {
        let mut context = md5::Context::new();
        context.consume(input);
        test::black_box(context.compute());
    });
}
