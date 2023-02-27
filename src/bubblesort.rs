use std::array;

fn bubblesort(array: &mut [i32]) -> &[i32]{
   
    for i in 0..array.len() {
        for j in 0..array.len() - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    array
}

mod tests {
    use super::bubblesort;

    #[test]
    fn test_bubblesort() {
        let mut array = [10, 7, 8, 9, 1, 5];
        let sorted = bubblesort(&mut array);
        assert_eq!(sorted, &[1, 5, 7, 8, 9, 10]);
    }
}