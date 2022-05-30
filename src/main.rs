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
    let mut it = v.iter();
    for _ in 0..k {
        it.next()?;
    }
    it.next()
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
    let r: Vec<Vec<&T>> = Vec::new();
    let (r, _) = v.iter().fold((r, None),
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

const V_EMPTY: &[i32] = &[];
const V_SINGLE: &[i32] = &[7];
const V_DOUBLE: &[i32] = &[12, 3];
const V_LONG: &[i32] = &[32, 17, 23, 9, 14, 6, 27, 18, 2];
const V_PALINDROME: &[char] = &['d', 'e', 'a', 'd', 'b', 'e', 'b', 'd', 'a', 'e', 'd'];
const V_NESTED: &[&[i32]] = &[&[3, 7, 19], &[9, 4], &[81, 34, 16, 56], &[0]];
const V_DUPS: &[i32] = &[1, 2, 2, 3, 4, 4, 4, 5, 5, 5, 6, 7, 8, 8, 8, 8, 9, 9, 10];

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
}
