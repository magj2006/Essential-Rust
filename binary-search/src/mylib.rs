use std::convert::AsRef;

pub fn find<T>(array: impl AsRef<[T]>, key: T) -> Option<usize>
where
    T: Ord,
{
    if array.as_ref().is_empty() {
        return None;
    }
    let mut l = 0 as isize;
    let mut r = (array.as_ref().len() - 1) as isize;

    while l <= r {
        let mid = (l + r) / 2;

        if array.as_ref()[mid as usize] == key {
            return Some(mid as usize);
        }

        if array.as_ref()[mid as usize] < key {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    None
}
