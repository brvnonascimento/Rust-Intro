fn main() {
    let age_old = 6;

    if age_old == 5 {
        println!("Go to Kindergarten");
    } else if (age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", (age_old - 5));
    } else if (age_old <= 25) && (age_old > 18) {
        println!("Go to College");
    } else {
        println!("Do what you want");
    }

    println!("!true = {}", !true);
    println!("true || false = {}", true || false);
    println!("true != false : {}", (true != false));

    let can_vote = if age_old >= 18 {true} else {false};
    println!("Can vote: {}", can_vote);
}