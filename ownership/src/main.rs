fn main() {
    let x = Box::new(1);
    let r1 = &x;
    let b = **r1;
    println!("{b}"); // 1
    // adding 1 with funny stuff
    let mut c = b;
    c += 1;
    let r2 = &c;
    println!("{}", *r2); // 2

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("third element is {}", num);
    v.push(4);
    // println!("again, third element is {}", *num);
    // this does not compile, uncomment to see why (or guess)
}
