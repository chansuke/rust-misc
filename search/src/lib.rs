pub fn jump_search<T: Eq + PartialOrd + Clone>(
    haystack: &[T],
    needle: &T,
    jump_size: usize,
) -> Option<usize> {
    if jump_size < haystack.len() {
        let mut i = 0;
        while i < haystack.len() - 1 {
            if i + jump_size < haystack.len() {
                i += jump_size
            } else {
                i = haystack.len() - 1;
            }
            if &haystack[i] == needle {
                return Some(i);
            } else if &haystack[i] > needle {
                return linear_search(&haystack[(i - jump_size)..i], needle);
            }
        }
    }
    None
}

pub fn binary_search<T: Eq + PartialOrd>(
    haystack: &[T],
    needle: &T,
) -> Option<usize> {
    let (mut left, mut right) = (0, haystack.len() - 1);
    while left <= right {
        let pivot = left + (right - left) / 2;
        if needle < &haystack[pivot] {
            right = pivot - 1;
        } else if needle > &haystack[pivot] {
            left = pivot + 1;
        } else {
            return Some(pivot);
        }
    }
    None
}

