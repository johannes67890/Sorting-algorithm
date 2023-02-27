mod quciksort;

fn main() {
    let mut ARRAY = [5, 2, 5, 1, 5];

    quciksort::quicksort(&mut ARRAY);
    // let array = Array(ARRAY);
    // array.if_in_array(1);
    // array.sum_of_array(&ARRAY);
    // array.high_low_of_array(&ARRAY);

    //let arr = quick_sort(&mut ARRAY);
    //println!("{:?}", &arr);
}
trait array_manipulation {
    fn if_in_array(&self, value: i32);
    fn sum_of_array(&self, array: &[i32]);
    fn high_low_of_array(&self, array: &[i32]) {println!("The max value in the array is {}, and the min is {}", array.iter().max().unwrap(), array.iter().min().unwrap());}
}

struct Array([i32; 5]);

impl array_manipulation for Array {
    fn if_in_array(&self, value: i32)  {
        let mut is_in_array = false;
        for i in 0..self.0.len() {
            if self.0[i] == value {
                is_in_array = true;
            }
        }
        if is_in_array {
            println!("{} is in the array", value);
        } else {
            println!("{} is not in the array", value);
        }
    }

    fn sum_of_array(&self, array: &[i32]) {
        let mut sum = 0;
        for i in 0..array.len() {
            sum += array[i];
        }
        println!("Sum of the array is: {sum}");
    }

    

}
