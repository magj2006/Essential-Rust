use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);

    let mut hs: HashSet<[u32; 3]> = HashSet::new();

    let ori_triples = vec![
        [3, 4, 5],
        [5, 12, 13],
        [8, 15, 17],
        [7, 24, 25],
        [20, 21, 29],
        [9, 40, 41],
        [12, 35, 37],
        [11, 60, 61],
        [28, 45, 53],
        [33, 56, 65],
        [16, 63, 65],
    ];

    (1 .. sum / 3).map(|x| ori_triples.iter().filter(|x| ori_triples.iter().any(|y| x * (y[0] + y[1] + y[2] == sum))
        .
    hs
}
