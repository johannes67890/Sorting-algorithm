mod bubble_sort;
mod quick_sort;
fn main(){
    // Arrays to sort
    let mut strArr = ["a","c","b","q","e"];
    let mut intArr = [8,5,9,2,7]; 
    // Bubble sort
    bubble_sort::bubblesort(&mut strArr); // sort the array
    println!("Bubble sort: {:?}", strArr); // OUTPUT: ["a", "b", "c", "e", "q"]

    // Quick sort
    quick_sort::quicksort(&mut intArr); 
    println!("Quick sort: {:?}", intArr); // OUTPUT: [2, 5, 7, 8, 9]
}