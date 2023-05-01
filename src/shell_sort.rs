/// Shell sort
/// 
/// # Algorithm
/// Shell sort is a variation of insertion sort.
/// Shell sort uses a gap value to create sublists.
/// The gap value starts with a larger value and gets smaller each time through the loop until it becomes 1.
/// Once the gap is 1, the list is then sorted using insertion sort.
/// 
/// # Time complexity
/// * Best case: O(n)
/// * Average case: O(n^2)
/// * Worst case: O(n^2)
/// 
/// # Ilustration
/// ``` ignore
/// arr = [ 8, 5, 9, 2, 7 ]
/// 
/// gap = 5 / 2 = 2
/// 
/// [ 8, 5, 9, 2, 7 ] -> [ 5, 8, 2, 7, 9 ]
/// 
/// gap = 2 / 2 = 1
/// 
/// [ 5, 8, 2, 7, 9 ] -> [ 2, 5, 7, 8, 9 ]
/// ```

pub fn shellsort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    let mut gap = len / 2;
    while gap > 0 {
        for i in gap..len {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}