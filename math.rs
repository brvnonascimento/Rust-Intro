fn main() {
    println!("{:.2}", 1.234);
    println!("B: {:b} H: {:x} 0: {:o}", 42, 10, 10);

    println!("{ten:>ws$}", ten=10, ws=5);
    println!("{ten:>0ws$}", ten=10, ws=5);

    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    let neg_4 = -4i32;

    println!("abs(-4) = {}", neg_4.abs());
}