
fn main() {
    _check_if_contains();
}

fn _splitting() {
    let rand_string = "I am a random a string!";

    println!("Length: {}", rand_string.len());
    let (first, second) = rand_string.split_at(6);

    println!("First: {}, Second: {}", first, second);
}

fn _iterating_chars() {
    let rand_string = "I am a random a string!";
    
    let count = rand_string.chars().count();

    let mut chars = rand_string.chars();

    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

    println!("Length: {}", count);
}

fn _iterating_words() {
    let rand_string = "I am a random a string!";

    let mut iter = rand_string.split_whitespace();

    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_word = iter.next();
    }
}

fn _iterating_lines() {
    let rand_string2 = "I am a random
    string\nThere are other strings like this it \nThis
    string is the best!";

    let mut lines = rand_string2.lines();
    let mut indiv_line = lines.next();

    loop {
        match indiv_line {
            Some(x) => println!("{}", x),
            None => break,
        }

        indiv_line = lines.next();
    }
}

fn _check_if_contains() {
    let rand_string2 = "I am a random
    string\nThere are other strings like this it \nThis
    string is the best!";

    println!("Find Best: {}", rand_string2.contains("best"))
}