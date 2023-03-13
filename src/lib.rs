mod bubblesort;
mod quicksort;

pub use bubblesort::bubblesort;
pub use quicksort::quicksort;

use rand::Rng;
use std::{ops::Range, ops::RangeBounds, ops::Bound};
pub fn generate_random_array(salt: i8, range: Range<i32>) -> [u8; 128]{
    match (range.start_bound(), range.end_bound()){
        (Bound::Included(start), Bound::Excluded(end)) => { // TODO: should only be included
            if start >= end {
                panic!("Start bound must be less than end bound"); // 3..0
            }
        },
        _=> panic!("Unsupported rangebound") // marcro: unimplemented!() (Error if chase is not covered)

    }
    
    let mut arr = [0u8; 128];
    let mut rng = rand::thread_rng();
    rng.fill(&mut arr);  

    arr
}


#[cfg(test)]
mod tests {
    use super::*;
    
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
}