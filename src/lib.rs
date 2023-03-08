mod bubblesort;
mod quciksort;

pub use bubblesort::bubblesort;
pub use quciksort::quicksort;

use rand::Rng;
use std::ops::RangeBounds;

pub fn generate_random_array<R: RangeBounds<i32>>(salt: i8, range: R) -> [u8; 128]{
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