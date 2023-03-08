use rand::Rng;
use std::ops::RangeBounds;

pub fn generate_random_array<R: RangeBounds<i32>>(salt: i8, range: R){
    let mut arr = [0u8; 128];
    let mut rng = rand::thread_rng();
    rng.fill(&mut arr);  

}
fn main(){
    generate_random_array(4,0..5);
}