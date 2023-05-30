/// 
/// # Quicksort
/// 
/// The Quicksort algoritm is a Divide and Conquer algorithm. 
/// Quicksort selects a `pivot` element, and takes the other elements smaller or equal on one side and bigger on the other side.
/// 
/// # Time complexity
/// * Best case: O(n log(n))
/// * Average case: O(n log(n))
/// * Worst case: O(n^2)
/// 
/// # Ilustration 
/// ``` ignore
/// arr = [ 8, 5, 9, 2, 7 ]` (pivot is the last element, in this case 7).
/// 
///                <-- less or equal           bigger than -->
/// 
///                              [ 8, 5, 9, 2, 7 ]
///                                      |
///                      / ------------- 7 ----------------\ 
///                  [ 5, 2 ]                            [ 8, 9 ]
///                      |                                  |
///               [ ] -- 2 -- [5]                    [8] -- 9 -- [ ]   
///                      |                                  |      
///                      \-----    [ 2, 5, 7, 8, 9 ]   ---- /
///                                  (Sorted array)
/// ```
/// 
pub fn quicksort<T: Ord>(array: &mut [T]) { // Takes mut array of type 'T'.
    let low: isize = 0; // Index '0' of array
    let high: isize = (array.len() - 1) as isize; // Highest index of array
   
    sort(array, low, high);

}


fn sort<T: Ord>(arr: &mut [T], low: isize, high: isize){
    if low < high {
        let pivot_element: isize = temp(arr, low, high); // pivot element (always the element of the highest index, in the whole or sub array)

        sort(arr, low, pivot_element - 1); // Before pivot
        sort(arr, pivot_element + 1, high); // After pivot
    }
    
    fn temp<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize; // Pivot element
    let mut i = low - 1; // Index of smaller element
    let mut j = high; // Index of bigger element

    loop {
            i += 1;
            while arr[i as usize] < arr[pivot] { // If current element is bigger than pivot element.
                i += 1;
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot] { // If current element is smaller than pivot element.
                j -= 1;
            }  
            if i >= j { // if pivot element is bigger than current element.
                break;
            } else {
                arr.swap(i as usize, j as usize); // Swap current element with pivot element.
            }
        }
        arr.swap(i as usize, pivot as usize); // Swap pivot element with current element.
        i
    }
}
