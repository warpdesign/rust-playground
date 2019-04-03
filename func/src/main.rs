fn main() {
    let mut x:i32 = 15;
    another_function(x, 6);
    println!("after call {}", x);
}

fn another_function(mut x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x = 5;
    println!("The value of x (2) is: {}", x);    
}