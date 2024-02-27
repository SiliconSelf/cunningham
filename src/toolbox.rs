//! Various helper functions used in the program

/// A basic recursive implementation of [Levenshtein Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// This function is recursive and generally inefficient
pub(crate) fn levenshtein_distance(a: &str, b: &str) -> usize {
    // |a| if |b| = 0
    if a.is_empty() {
        return a.len();
    }
    // |b| if |a| = 0
    if b.is_empty() {
        return b.len();
    }
    let a_head = a.get(0..1).expect("Can't get a head");
    let a_tail = a.get(1..).expect("Can't get a tail");
    let b_head = b.get(0..1).expect("Can't get b head");
    let b_tail = b.get(1..).expect("Can't get b tail");
    // lev(tail(a), tail(b)) if head(a) = head(b)
    if a_head == b_head {
        return levenshtein_distance(a_tail, b_tail);
    }
    // 1+min Otherwise
    let options: [usize; 3] = [
        levenshtein_distance(a_tail, b),
        levenshtein_distance(a, b_tail),
        levenshtein_distance(a_tail, b_tail),
    ];
    let mut options_min = options[0];
    options.map(|x| {
        if x < options_min {
            options_min = x
        }
    });
    1 + options_min
}
