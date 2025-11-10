fn main() {
    // immutable variable
    let a = 10;
    //mutable variable 
    let mut b = 12;
    // constant 
    const PI :f32 = 3.141592653589793;
    println!("immutable {}, mutable {}, constant {}", a, b, PI);
    b = 23;
    println!("mutable {}", b);
}
