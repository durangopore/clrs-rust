pub fn insertion_sort<'a, T>(keys: &'a mut [T])
where
    T: PartialOrd + Clone,
{
    // shamelessly plagiarized from https://rosettacode.org/wiki/Sorting_algorithms/Insertion_sort#Rust
    // for current in 1..keys.len() {
    //     let mut prev = current;
    //     while prev > 0 && keys[prev] < keys[prev - 1] {
    //         keys.swap(prev, prev - 1);
    //         prev -= 1;
    //     }
    // }

    // shamelessly plagiarized from https://stackoverflow.com/a/44600937/76813
    for current in 1..keys.len() {
        let key = keys[current].clone();
        let mut prev = current;
        while prev > 0 && keys[prev - 1] > key {
            keys[prev] = keys[prev - 1].clone();
            prev -= 1;
        }
        keys[prev] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_array = [0u8; 0];
        let copy = empty_array.clone();
        insertion_sort(&mut empty_array);
        assert_eq!(copy, empty_array)
    }

    #[test]
    fn sorted() {
        let mut sorted_array = [1, 2, 3];
        let copy = sorted_array.clone();
        insertion_sort(&mut sorted_array);
        assert_eq!(copy, sorted_array)
    }

    #[test]
    fn reverse() {
        let mut reverse_array = [3, 2, 1];
        insertion_sort(&mut reverse_array);
        assert_eq!([1, 2, 3], reverse_array)
    }

    #[test]
    fn unsorted() {
        let mut unsorted_array = [1, 3, 2];
        insertion_sort(&mut unsorted_array);
        assert_eq!([1, 2, 3], unsorted_array)
    }

    #[test]
    fn strings() {
        let mut string = ["i", "n", "s", "e", "r", "t", "i", "o", "n"];
        insertion_sort(&mut string);
        assert_eq!(["e", "i", "i", "n", "n", "o", "r", "s", "t"], string)
    }
}
