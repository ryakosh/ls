pub fn count_digits(num: u64) -> usize {
    if num < 10 {
        1
    } else if num < 100 {
        2
    } else if num < 1000 {
        3
    } else if num < 10000 {
        4
    } else if num < 100000 {
        5
    } else if num < 1000000 {
        6
    } else if num < 10000000 {
        7
    } else if num < 100000000 {
        8
    } else if num < 1000000000 {
        9
    } else if num < 10000000000 {
        10
    } else if num < 100000000000 {
        11
    } else { // No file is that big right?... right?
        12
    }
}