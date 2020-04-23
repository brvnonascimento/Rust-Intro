fn main() {
    let rand_tuple = ("Derek", 40);

    let rand_tuple_2: (&str, i8) = ("Derek", 40);

    println!("Name: {}", rand_tuple_2.0);
    println!("Age: {}", rand_tuple.1);
}