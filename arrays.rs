fn main() {
    let rand_array = [1,2,3];

    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());

    println!("Second 2: {:?}", &rand_array[1..3]);
    println!("Whole array: {:?}", &rand_array[0..rand_array.len()]);
}