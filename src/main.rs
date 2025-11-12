fn main() {
    // // immutable variable
    // let a = 10;
    // //mutable variable
    // let mut b = 12;
    // // constant
    // const PI :f32 = 3.141592653589793;
    // println!("immutable {}, mutable {}, constant {}", a, b, PI);
    // b = 23;
    // println!("mutable {}", b);
    // // shadowing :D
    // // the previous variable 'a' is shadowed by a new variable 'a'
    // let a = "shadowed";
    // // the compiler treats this as a new variable
    // println!("shadowed {}", a);

    // day 4
    //
    //shadowing scope
    let x = 10;
    println!("{}", x);
    let x = "342";
    println!("{}", x);
    {
        let x  = 235;
        println!("{}", x);
    }
    println!("{}" , x)

    // day 5 Chapter 3 3.2 Datatypes :(

}
