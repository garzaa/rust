fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("X is now {}", x);

    {
        let x = x + 1;
        println!("x is now {}", x);
    }
    println!("x is now {}", x);

    let y = 2;
    let y = 3;


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{:?}", tup);

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
