
/// # Quicksort
/// 
/// The Quicksort algoritm is a Divide and Conquer algorithm. 
/// Quicksort selects a `pivot` element, and takes the other elements smaller or equal on one side and bigger on the other side.
/// 
/// # Ilustration 
/// `arr = [ 8, 5, 9, 2, 7 ]` (pivot is the last element, in this case 7).
/// 
///                <-- `less or equal`      `bigger than` -->
/// 
///                              [ 8, 5, 9, 2, 7 ]
///                                      |
///                      / ------------- 7 ----------------\ 
///                  [ 5, 2 ]                            [ 8, 9 ]
///                      |                                  |
///               [ ] -- 2 -- [5]                    [8] -- 9 -- [ ]   
///                      |                                  |            <---- `Now we join them back together`
///                      \-----    [ 2, 5, 7, 8, 9 ]   ---- /
///                                  (Sorted array)
///          
pub fn quicksort<T: Ord>(array: &mut [T]) { // Takes mut array of type 'T'.
    let low: isize = 0; // Index '0' of array
    let high: isize = (array.len() - 1) as isize; // Highest index of array
   
    sort(array, low, high);

}


fn sort<T: Ord>(arr: &mut [T], low: isize, high: isize){
    if low < high {
        let pivot_element: isize = temp(arr, low, high); // pivot element (always the element of the highest index, in the whole or sub array)

        sort(arr, low, pivot_element - 1); // Before pi
        sort(arr, pivot_element + 1, high); // After pi
    }
    
    fn temp<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize; // Pivot element
    let mut i = low - 1; 
    let mut j = high;

    loop {
            i += 1;
            while arr[i as usize] < arr[pivot] {
                i += 1;
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot] {
                j -= 1;
            }  
            if i >= j { // If current element is smaller than pivot element.
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap(i as usize, pivot as usize);
        i
    }
}
