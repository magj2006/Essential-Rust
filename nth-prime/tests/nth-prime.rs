#![feature(test)]

extern crate test;

use nth_prime as np;
use test::Bencher;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[bench]
fn bench_test_big_prime(b: &mut Bencher) {
    b.iter(|| test_big_prime());
}
