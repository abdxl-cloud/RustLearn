fn add (i: i32, i2:i32) -> i32{
    i + i2
}

fn main() {
    let x = 5; // immutable by default
    println!("The value of x is: {}", x);

    // mutable variable
    let mut y = 10;
    println!("Original y: {}", y);
    y = 20;
    println!("Updated y: {} + {}", y, x);

    // Defining a variable
    let integer:i32 = 21;
    let integer2:u32 = 22;
    let integer3:i64 = 23;

    let float:f32 = 45.5;
    let float:f64 = 500.5432;

    let is_adult:bool = false;
    let character:char = 'R'; 
    println!("{} {}", integer, is_adult);


    // Compound Datatype
    let tup = (33, 34.5, "hello");
    let (x1, x2, x3) = tup;

    let array = [1, 2, 3, 4, 5];

    let result = add(5, 7);
    println!("The sum is {}", result);
}
