pub fn mergesort<T: Ord>(arr: T) {
    let left: isize = 0;
    let right: isize = (array.len() - 1) as isize; // Highest index of array

    if(left >= right) {
        return;
    }
    let mid: isize = (left + right) / 2; // Middle index of array

    sort(arr, left, mid);
    sort(arr, mid + 1, right);
    merge(arr, left, mid, right);
}

fn sort<T: Ord>(arr: T, left: isize, right: isize) {

}

fn merge<T: Ord>(arr: T, left: isize, mid: isize, right: isize) {

}