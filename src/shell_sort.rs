/// Shell sort
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
/// 
/// # Complexity
/// 
/// | Best | Average | Worst |
/// |------|---------|-------|
/// | O(n) | O(n^2)  | O(n^2)|
/// 
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