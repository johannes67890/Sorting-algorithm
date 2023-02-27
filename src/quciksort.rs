pub fn quicksort(array: &mut [i32]) -> &[i32] {
    let low: isize = 0;
    let high: isize = (array.len() - 1) as isize;
   
   sort(array, low, high);


    fn sort(array: &mut [i32], low: isize, high: isize) -> &[i32]{
        if low < high {
            let pi: isize = temp(array, low, high);

            sort(array, low, pi - 1);
            sort(array, (pi + 1) as isize, high);
         }
         array
    }

    fn temp(array: &mut [i32], low: isize, high: isize) -> isize {
        let temp = array[high as usize];
        let mut i = low - 1; 

        for j in 0..high {
            if array[j as usize] < temp {
                i += 1;

                array.swap(i as usize, j as usize);

            }
        }
       array.swap((i + 1) as usize, high as usize);
       i + 1
    }

    array
}

mod tests {
    use super::quicksort;

    #[test]
    fn test_quicksort() {
        let mut array = [10, 7, 8, 9, 1, 5];
        let sorted = quicksort(&mut array);
        assert_eq!(sorted, &[1, 5, 7, 8, 9, 10]);
    }
}
