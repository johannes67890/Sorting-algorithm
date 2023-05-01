/// Insertion sort algorithm
/// 
/// The Insertion sort algorithm is a simple sorting algorithm that builds the final sorted array (or list) one item at a time.
/// 
/// # Time complexity
/// * Best case: O(n)
/// * Average case: O(n^2)
/// * Worst case: O(n^2)
/// 
/// # Ilustration
/// ```` ignore
/// arr = [ 8, 5, 9, 2, 7 ]
/// 
/// [ 8, 5, 9, 2, 7 ] -> [ 5, 8, 9, 2, 7 ] -> [ 5, 8, 9, 2, 7 ] -> [ 2, 5, 8, 9, 7 ] -> [ 2, 5, 7, 8, 9 ]
/// ````
pub fn insertionsort<T: Ord>(array: &mut [T]) {
    let mut n = array.len();
    for i in 1..n {
        let mut j = i;
        // move elements of array[0..i-1], that are greater than key, to one position ahead of their current position
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
} 
