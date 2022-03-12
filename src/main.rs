use std::slice::Iter;

// P01: find the last element of a list.
fn find_last<'a, T>(v: &'a [T]) -> Option<&'a T> {
    v.iter().reduce(|_, e| e)
}

// P02: find next to last element of a list.
fn find_next_to_last<'a, T>(v: &'a [T]) -> Option<&'a T> {
    let (r, _) = v.iter().fold((None, None), |a, e| (a.1, Some(e)));
    r
}

// P03: find k-th element of a list.
fn find_kth<'a, T>(v: &'a [T], k: usize) -> Option<&'a T> {
    fn find<'a, T>(mut it: Iter<'a, T>, n: usize) -> Option<&'a T> {
        match (it.next(), n) {
            (None, _) => None,
            (r, 0) => r,
            _ => find(it, n - 1),
        }
    }
    find(v.iter(), k)
}

// P04: find number of elements in a list.
fn find_length<'a, T>(v: &'a [T]) -> usize {
    v.iter().fold(0, |a, _| a + 1)
}

const V_EMPTY: &[i32] = &[];
const V_SINGLE: &[i32] = &[7];
const V_DOUBLE: &[i32] = &[12, 3];
const V_LONG: &[i32] = &[32, 17, 23, 9, 14, 6, 27, 18, 2];

fn main() {
    println!("--- P01 ---");
    let r = find_last(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = find_last(V_LONG);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P02 ---");
    let r = find_next_to_last(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = find_next_to_last(V_SINGLE);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = find_next_to_last(V_DOUBLE);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = find_next_to_last(V_LONG);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P03 ---");
    let r = find_kth(V_EMPTY, 2);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = find_kth(V_SINGLE, 0);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = find_kth(V_LONG, 6);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P04 ---");
    let r = find_length(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = find_length(V_SINGLE);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = find_length(V_DOUBLE);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = find_length(V_LONG);
    println!("{:?} -> {:?}", V_LONG, r);
}
