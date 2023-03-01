pub fn quicksort<T: Ord>(array: &mut [T]) {
    let low: isize = 0;
    let high: isize = (array.len() - 1) as isize;
   
    sort(array, low, high);

    fn sort<T: Ord>(array: &mut [T], low: isize, high: isize){
        if low < high {
            let pi: isize = temp(array, low, high);

            sort(array, low, pi - 1);
            sort(array, (pi + 1) as isize, high);
         
    }

    fn temp<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
        let mut i = low - 1; 

        for j in 0..high {
            if &array[j as usize] < &array[high as usize] {

                i += 1;

                array.swap(i as usize, j as usize);

            }
        }
       array.swap((i + 1) as usize, high as usize);
       i + 1
    }
    }
}
