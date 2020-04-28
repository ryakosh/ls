pub fn count_digits(num: u64) -> usize {
    if num < 10 {
        1
    } else if num < 100 {
        2
    } else if num < 1_000 {
        3
    } else if num < 10_000 {
        4
    } else if num < 100_000 {
        5
    } else if num < 1_000_000 {
        6
    } else if num < 10_000_000 {
        7
    } else if num < 100_000_000 {
        8
    } else if num < 1_000_000_000 {
        9
    } else if num < 10_000_000_000 {
        10
    } else if num < 100_000_000_000 {
        11
    } else {
        // No file is that big right?... right?
        12
    }
}
