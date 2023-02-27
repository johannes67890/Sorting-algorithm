mod quciksort;
mod bubblesort;

fn main() {
    let mut ARRAY = [5, 2, 5, 1, 5];

    quciksort::quicksort(&mut ARRAY);
}

mod sorts {
    pub mod bubblesort;
    pub mod quicksort;
}

