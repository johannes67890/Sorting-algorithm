use std::vec;


pub fn mergesort<T: Ord>(arr: &mut [T]) {
    sort(arr, 0, (arr.len() - 1));
}

fn sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {

    if(left >= right) { // recursive 
        return;
    }
    let mid: usize = (left + right) / 2; // Middle index of array

    sort(arr, left, mid);
    sort(arr, mid + 1, right);
    merge(arr, left, mid, right);
}

fn merge<T: Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) {


    let n1 = mid - left + 1;
    let n2 = right - mid;

    let left_arr: Vec<&T> = (0..n1).into_iter().map(|i| &arr[left+i] ).collect();

   let right_arr: Vec<&T> = (0..n2).into_iter().map(|i| &arr[mid + 1 + i] ).collect();

    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut k:usize = left;

    while i < n1 && j < n2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }  

}