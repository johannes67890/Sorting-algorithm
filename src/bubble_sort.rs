 /// Bubble sort
 /// 
 /// Bubble sort swaps each pair of adjacent items if they are in the wrong order.
 /// repeat until no swaps are needed.
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
 pub fn bubblesort<T: Ord>(arr: &mut [T]) { // Generic function that accepts any type that implements the Ord trait
    // arr is a mutable reference to a slice of type T
    let len = arr.len(); // get the length of the array
    for i in 0..len { // iterate over the array
        for j in 0..len - i - 1 { 
            if arr[j] > arr[j + 1] { // swap if the current value is greater than the next value
                arr.swap(j, j + 1);
            }
        }
    }
}


