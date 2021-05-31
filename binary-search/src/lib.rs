use std::cmp::Ordering::{Equal, Greater, Less};
use std::convert::AsRef;
use std::fmt::Debug;

pub fn find<T: Ord + Debug>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    let array = array.as_ref();

    let mid = array.len() / 2;

    dbg!(&array, mid);

    /*
    When the algorithm determines that the value we're looking
    for is in the last half of the slice, we run the algorithm
    again on a new slice containing only the last half of the original slice.
    The new slice knows nothing about the indexes of the values in the original slice,
    so if the value is found in it, the returned index will refer to its position in the new slice.

    Here's the trick: since we know how far into the original slice the new slice started,
    we can just add that offset to the returned index (if there is one) to get the original index!
    That offset is encoded in the signature of the new slice: &space[mid + 1..].
    The value at index mid + 1 in the original slice will have index 0 in the new slice,
    so to get the original index, we have to add mid + 1.

    Because the return value is an Option, we can't just add the offset to it directly;
    instead, we use map to apply a function to the returned value if there is one.
    That function needs to map an index from the new slice to its index in the original slice.
    We found in the previous paragraph that this is easily done by adding mid + 1. In this case,
    we use the closure |i| i + mid + 1.
    */
    match key.cmp(array.get(mid)?) {
        Equal => Some(mid),
        Less => find(&array[..mid], key),
        Greater => find(&array[mid + 1..], key).map(|i| {
            dbg!(mid, i, mid + i + 1);
            mid + i + 1
        }),
    }
}
