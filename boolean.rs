fn main() {
    let is_it_true: bool = true;

    let let_x: char = 'x';

    let (f_name, l_name) = ("Derek", "Banas");

    println!("My name is {} {}, it is {}", f_name, l_name, is_it_true);
    
    println!("Is it {0} that {1} is {0}", is_it_true, let_x);
}