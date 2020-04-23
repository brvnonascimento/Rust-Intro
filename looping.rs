fn main() {
    let mut x = 1;

    loop {
        if (x % 2) == 0 {
            println!("{}", x);
            x += 1;

            continue;
        }

        if x > 10 {
            break;
        }

        x += 1;
        continue;
    }

    let mut y = 1;

    while y <= 10 {
        println!("{}", y);
        y += 1;
    }

    for z in 1..10 {
        println!("FOR: {}", z);
    }
}