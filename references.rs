fn main () {
    //it works cuz primitive values
    // let prim_val = 1;
    // let prim_val2 = prim_val;
    // println!("prim_val: {}", prim_val);
    
    let vect1 = vec![1,2,3];
    //it doesn't work cuz you can't acess a copy of a vector
    // let vect2 = vect1;
    // println!("vect1[0]: {}", vect1[0]);

    println!("Sum of vectors: {}", sum_vects(&vect1));
    println!("Vect: {:?}", vect1);
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});

    return sum;
}