/// Merge Sort
/// 
/// Merge sort is a divide and conquer algorithm.
/// 
/// The pivot point is the middle index of array. It is calculated by `(left index + right index) / 2`.
/// The array is divided into two subarrays, left and right.
/// The left subarray is sorted by recursively calling the merge sort function.
/// The right subarray is sorted by recursively calling the merge sort function, after the left subarray is sorted.
/// At the end, the two subarrays are merged into one sorted array.
/// 
/// # Time complexity
/// * Best case: O(n log n)
/// * Average case: O(n log n)
/// * Worst case: O(n log n)
/// 
/// # Ilustration 
/// ``` ignore
/// arr = [ 8, 5, 9, 2, 7 ]`
///       if left index < right index -> pivot point = (left index + right index) / 2
///                 
///                   <-- Left Index          Right Index -->
/// 
///                              [ 8, 5, 9, 2, 7 ]        --> pivot point = 2
///                                      |                                
///                       / -------------------------------\ 
///                  [ 8, 5, 9 ]                         [ 2, 7 ]
///                      |                                  |
///           [ 8, 5 ] ------ [9]                     [2] ----- [7]   
///                      |                                  |      
///          [ 8 ] -- [ 5 ] -- [  ]                         |            -> sort and merge the two subarrays into one sorted array
///                     |                                   |
///          [ 5 ] -- [ 8 ] -- [ 9 ]                 [ 2 ] -- [ 7 ]
///                     |                                   |
///            [ 5, 8 ] -- [ 9 ]                         [ 2, 7 ]
///                     |                                   |
///                [ 5, 8, 9 ]                              |
///                     |                                   |
///                      \-----    [ 2, 5, 7, 8, 9 ]   ---- /
///                                 
/// ```
/// [See detailed illustration of Merge sort in action](https://media.geeksforgeeks.org/wp-content/uploads/20220722205737/MergeSortTutorial.png)
/// 
pub fn mergesort<T: Ord + Clone>(arr: &mut [T]) {
    sort(arr, 0, arr.len() - 1);
}

fn sort<T: Ord + Clone>(arr: &mut [T], left: usize, right: usize) {

    if left >= right { // recursive 
        return;
    }
    let mid: usize = (left + right) / 2; // Middle index of array

    // Divide array into two subarrays, left first then right subarray
    sort(arr, left, mid);
    sort(arr, mid + 1, right);
    // Merge the two subarrays into one sorted array
    merge(arr, left, mid, right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let n1 = mid - left + 1; // size of left subarray
    let n2 = right - mid; // size of right subarray

    // Create two temporary arrays of size n1 and n2 (left and right subarrays)
    let left_arr: Vec<T> = (0..n1).into_iter().map(|i| arr[left+i].clone() ).collect(); 
    let right_arr: Vec<T> = (0..n2).into_iter().map(|i| arr[mid + 1 + i].clone() ).collect();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = left;

    // Merge the two subarrays into one sorted array
    while i < n1 && j < n2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i].clone();
            i += 1;
        } else {
            arr[k] = right_arr[j].clone();
            j += 1;
        }
        k += 1;
    }
    // Copy the remaining elements of left_arr, if there are any
    while i < n1 {
        arr[k] = left_arr[i].clone();
        i += 1;
        k += 1;
    }
    // Copy the remaining elements of right_arr, if there are any
    while j < n2 {
        arr[k] = right_arr[j].clone();
        j += 1;
        k += 1;
    }  

}