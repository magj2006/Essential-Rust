use std::collections::BTreeMap;
//use std::slice;
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&k, v)| v.iter().map(move |x| (x.to_ascii_lowercase(), k)))
        .collect()
}
