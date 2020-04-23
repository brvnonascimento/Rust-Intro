fn main() {
    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7,8));

    let num_ten = 10;

    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));
}