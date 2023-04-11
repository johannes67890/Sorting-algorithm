mod bubble_sort;
mod quick_sort;
mod insertion_sort;
mod merge_sort;
mod shell_sort;

pub use bubble_sort::bubblesort;
pub use quick_sort::quicksort;
pub use insertion_sort::insertionsort;
pub use merge_sort::mergesort;
pub use shell_sort::shellsort;

use rand::Rng;
use std::{ops::Range, ops::RangeBounds, ops::Bound};
pub fn generate_random_array(range: Range<u32>, size: u32) -> Vec<u32> {
    match (range.start_bound(), range.end_bound()){
        (Bound::Included(start), Bound::Excluded(end)) => { // TODO: should only be included
            if start >= end {
                panic!("Start bound must be less than end bound"); // 3..0
            }
        },
        _=> panic!("Unsupported rangebound") // marcro: unimplemented!() (Error if chase is not covered)
    }

    let mut arr: Vec<u32> = Vec::with_capacity(size as usize);
    
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        arr.push(rng.gen_range(range.clone()));
    }
    
    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Unit tests for generate_random_array
    #[test]
    fn test_generate_random_array(){
        let arr = generate_random_array(0..100, 1000);
        assert_eq!(arr.len(), 1000);
    }

    ///
    /// Test sorting algorithms
    /// 

    #[test]
    fn test_bubble_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        bubblesort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
    #[test]
    fn test_quick_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
    #[test]
    fn test_insertion_sort(){
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        insertionsort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
    #[test]
    fn test_merge_sort(){
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        mergesort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
}