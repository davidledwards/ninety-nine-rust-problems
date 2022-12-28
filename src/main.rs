// Note that the implementation of each problem is not meant to be the most efficient,
// but rather an exercise is using functional constructs of the language. As such, you
// will see fold used quite extensively.

use std::ops::ControlFlow;

// P01: find the last element of a list.
fn find_last<T>(v: &[T]) -> Option<&T> {
    v.iter().reduce(|_, e| e)
}

// P02: find next to last element of a list.
fn find_next_to_last<T>(v: &[T]) -> Option<&T> {
    let (r, _) = v.iter().fold((None, None), |a, e| (a.1, Some(e)));
    r
}

// P03: find k-th element of a list.
fn find_kth<T>(v: &[T], k: usize) -> Option<&T> {
    // This is intentionally overdone because it is designed to illustrate use of
    // early return from fold.
    let r = v.iter().try_fold(0, |i, e| {
        if i == k {
            ControlFlow::Break(e)
        } else {
            ControlFlow::Continue(i + 1)
        }
    });
    match r {
        ControlFlow::Break(e) => Some (e),
        _ => None
    }
}

// P04: find number of elements in a list.
fn find_length<T>(v: &[T]) -> usize {
    v.iter().fold(0, |a, _| a + 1)
}

// P05: reverse a list.
fn reverse<T>(v: &[T]) -> Vec<&T> {
    v.iter().rev().fold(Vec::with_capacity(v.len()),
        |mut r, e| {
            r.push(e);
            r
        })
}

// P06: determine if list is a palindrome.
fn is_palindrome<T: Ord>(v: &[T]) -> bool {
    let n = v.len();
    (0..n / 2).fold(true, |r, i| r && (v[i] == v[n - i - 1]))
}

// P07: flatten a nested list.
fn flatten<'a, T>(v: &[&'a [T]]) -> Vec<&'a T> {
    v.iter().fold(Vec::new(),
        |mut r, e| {
            e.iter().for_each(|e| r.push(e));
            r
        })
}

// P08: eliminate consecutive duplicates of list elements.
fn compress<T: Ord>(v: &[T]) -> Vec<&T> {
    let (r, _) = v.iter().fold((Vec::new(), None),
        |(mut r, p), e| match p {
            Some(v) if v == e =>
                (r, p),
            _ => {
                r.push(e);
                (r, Some(e))
            }
        });
    r
}

// P09: pack consecutive duplicates of list elements into sublists.
fn pack<T: Ord>(v: &[T]) -> Vec<Vec<&T>> {
    let (r, _) = v.iter().fold((Vec::<Vec<&T>>::new(), None),
        |(mut r, p), e| match p {
            Some(v) if v == e => {
                r.last_mut().unwrap().push(e);
                (r, p)
            }
            _ => {
                r.push(vec![e]);
                (r, Some(e))
            }
        });
    r
}

// P10: use P09 to run-length encode list of elements.
fn encode<T: Ord>(v: &[T]) -> Vec<(usize, &T)> {
    pack(v).iter().map(|e| (e.len(), e[0])).collect()
}

// P12: decode a run-length encoded list.
fn decode<T>(v: &[(usize, T)]) -> Vec<&T> {
    v.iter().fold(Vec::new(),
        |mut r, (n, e)| {
            (0..*n).for_each(|_| r.push(e));
            r
        })
}

// P14: duplicate elements of a list.
fn duplicate<T>(v: &[T]) -> Vec<&T> {
    v.iter().fold(Vec::new(),
        |mut r, e| {
            r.push(e);
            r.push(e);
            r
        })
}

// P15: duplicate elements of a list a given number of times.
fn duplicate_times<T>(v: &[T], n: usize) -> Vec<&T> {
    v.iter().fold(Vec::new(),
        |mut r, e| {
            (0..n).for_each(|_| r.push(e));
            r
        })
}

// P16: drop every n-th element of a list.
fn drop_nth<T>(v: &[T], n: usize) -> Vec<&T> {
    let (r, _) = v.iter().fold((Vec::new(), n),
        |(mut r, k), e| {
            if k == 1 {
                (r, n)
            } else {
                r.push(e);
                (r, k - 1)
            }
        });
    r
}

// P17: split a list into two parts.
fn split<T>(v: &[T], n: usize) -> (Vec<&T>, Vec<&T>) {
    let (a, b, _) = v.iter().fold((Vec::new(), Vec::new(), n),
        |(mut a, mut b, k), e| {
            let j = if k > 0 {
                a.push(e);
                k - 1
            } else {
                b.push(e);
                0
            };
            (a, b, j)
        });
    (a, b)
}

// P18: extract a slice from from a list.
fn slice<T>(v: &[T], i: usize, j: usize) -> &[T] {
    let n = v.len();
    if i < n {
        let k = if j > n { n } else { j };
        &v[i..k]
    } else {
        &[]
    }
}

// P19: rotate a list left by a given number of places.
fn rotate<T>(v: &[T], n: usize) -> Vec<&T> {
    let (mut r, ref mut t, _) = v.iter().fold((Vec::new(), Vec::new(), n),
        |(mut r, mut t, k), e| {
            let j = if k > 0 {
                t.push(e);
                k - 1
            } else {
                r.push(e);
                k
            };
            (r, t, j)
        });
    r.append(t);
    r
}

// P20: remoove k-th element from a list and return as a tuple.
fn remove_kth<T>(v: &[T], k: usize) -> (Vec<&T>, Option<&T>) {
    let (r, s, _) = v.iter().fold((Vec::new(), None, 0),
        |(mut r, s, j), e| {
            let _s = if j == k {
                Some(e)
            } else {
                r.push(e);
                s
            };
            (r, _s, j + 1)
        });
    (r, s)
}

// P21: insert element at given position in a list.
fn insert_at<'a, T>(v: &'a [T], t: &'a T, k: usize) -> Vec<&'a T> {
    if k < v.len() {
        let (r, _) = v.iter().fold((Vec::new(), 0), |(mut r, i), e| {
            if i == k {
                r.push(t)
            }
            r.push(e);
            (r, i + 1)
        });
        r
    } else {
        let mut r: Vec<&'a T> = v.iter().collect();
        r.push(t);
        r
    }
}

// P22: create list containing integers within a given range.
fn range_of(begin: u32, end: u32) -> Vec<u32> {
    let mut r = Vec::new();
    if begin < end {
        let mut i = begin;
        while i < end {
            r.push(i);
            i += 1
        }
    }
    r
}

const V_EMPTY: &[i32] = &[];
const V_SINGLE: &[i32] = &[7];
const V_DOUBLE: &[i32] = &[12, 3];
const V_LONG: &[i32] = &[32, 17, 23, 9, 14, 6, 27, 18, 2];
const V_PALINDROME: &[char] = &['d', 'e', 'a', 'd', 'b', 'e', 'b', 'd', 'a', 'e', 'd'];
const V_ORDERED: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const V_NESTED: &[&[i32]] = &[&[3, 7, 19], &[9, 4], &[81, 34, 16, 56], &[0]];
const V_DUPS: &[char] = &['a', 'b', 'b', 'c', 'd', 'd', 'd', 'e', 'e', 'e',
                         'f', 'g', 'h', 'h', 'h', 'h', 'i', 'i', 'j'];

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

    println!("--- P05 ---");
    let r = reverse(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = reverse(V_SINGLE);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = reverse(V_DOUBLE);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = reverse(V_LONG);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P06 ---");
    let r = is_palindrome(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = is_palindrome(V_SINGLE);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = is_palindrome(V_LONG);
    println!("{:?} -> {:?}", V_LONG, r);
    let r = is_palindrome(V_PALINDROME);
    println!("{:?} -> {:?}", V_PALINDROME, r);

    println!("--- P07 ---");
    let r = flatten(V_NESTED);
    println!("{:?} -> {:?}", V_NESTED, r);

    println!("--- P08 ---");
    let r = compress(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = compress(V_SINGLE);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = compress(V_DUPS);
    println!("{:?} -> {:?}", V_DUPS, r);

    println!("--- P09 ---");
    let r = pack(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = pack(V_DOUBLE);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = pack(V_DUPS);
    println!("{:?} -> {:?}", V_DUPS, r);

    println!("--- P10 ---");
    let r = encode(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = encode(V_DOUBLE);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = encode(V_DUPS);
    println!("{:?} -> {:?}", V_DUPS, r);

    println!("--- P12 ---");
    let v = encode(V_DUPS);
    let r = decode(&v);
    println!("{:?} -> {:?}", v, r);

    println!("--- P14 ---");
    let r = duplicate(V_EMPTY);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = duplicate(V_PALINDROME);
    println!("{:?} -> {:?}", V_PALINDROME, r);

    println!("--- P15 ---");
    let r = duplicate_times(V_EMPTY, 3);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = duplicate_times(V_SINGLE, 3);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = duplicate_times(V_PALINDROME, 3);
    println!("{:?} -> {:?}", V_PALINDROME, r);

    println!("--- P16 ---");
    let r = drop_nth(V_EMPTY, 1);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = drop_nth(V_SINGLE, 1);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = drop_nth(V_SINGLE, 2);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = drop_nth(V_ORDERED, 2);
    println!("{:?} -> {:?}", V_ORDERED, r);
    let r = drop_nth(V_ORDERED, 3);
    println!("{:?} -> {:?}", V_ORDERED, r);
    let r = drop_nth(V_ORDERED, 1);
    println!("{:?} -> {:?}", V_ORDERED, r);

    println!("--- P17 ---");
    let r = split(V_EMPTY, 1);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = split(V_SINGLE, 1);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = split(V_DOUBLE, 1);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = split(V_ORDERED, 7);
    println!("{:?} -> {:?}", V_ORDERED, r);

    println!("--- P18 ---");
    let r = slice(V_EMPTY, 0, 1);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = slice(V_LONG, 2, 5);
    println!("{:?} -> {:?}", V_LONG, r);
    let r = slice(V_LONG, 2, V_LONG.len());
    println!("{:?} -> {:?}", V_LONG, r);
    let r = slice(V_LONG, 2, V_LONG.len() + 1);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P19 ---");
    let r = rotate(V_EMPTY, 1);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = rotate(V_LONG, 0);
    println!("{:?} -> {:?}", V_LONG, r);
    let r = rotate(V_LONG, 3);
    println!("{:?} -> {:?}", V_LONG, r);
    let r = rotate(V_DUPS, V_DUPS.len() / 2);
    println!("{:?} -> {:?}", V_DUPS, r);

    println!("--- P20 ---");
    let r = remove_kth(V_EMPTY, 0);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = remove_kth(V_SINGLE, 0);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = remove_kth(V_LONG, 3);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P21 ---");
    let r = insert_at(V_EMPTY, &999, 0);
    println!("{:?} -> {:?}", V_EMPTY, r);
    let r = insert_at(V_SINGLE, &999, 1);
    println!("{:?} -> {:?}", V_SINGLE, r);
    let r = insert_at(V_DOUBLE, &999, 0);
    println!("{:?} -> {:?}", V_DOUBLE, r);
    let r = insert_at(V_LONG, &999, 5);
    println!("{:?} -> {:?}", V_LONG, r);

    println!("--- P22 ---");
    println!("{:?}", range_of(0, 1));
    println!("{:?}", range_of(3, 3));
    println!("{:?}", range_of(3, 2));
    println!("{:?}", range_of(7, 14));
}
