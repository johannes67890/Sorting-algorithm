use  sorting_algorithm::generate_random_array;
fn main(){
    let arr = generate_random_array(0..100, 10000);
    let mut j = 0;
    for i in arr.iter(){
      
        if i == &100 {
            j += 1;
        }
    }
    println!("found {} in array", j);
    println!("{:?}", arr)
}
