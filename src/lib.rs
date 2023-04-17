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

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn test_shell_sort(){
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        shellsort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
}