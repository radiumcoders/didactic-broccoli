fn main(){
    // let 
    let a = 1;
    // mut
    let mut b = 2;
    // const 
    const PI :f32 = 3.14159;
   println!("let {} mutable let {} const {}",a,b,PI); 
   b = 23;
   println!("mutable let {}", b);
   // day 6 data types 
   // int
   let small :u8 = 1;
   let big :u128 = 234232535235325324;
   let medium :u16 = 23423;
   let small2 : i8 = -1;
   let medium2 : i16 = -23423;
   let large : i64 = -234232535235325324;
   println!("int {} {} {} {} {} {}",small,big,medium,small2,medium2,large);
   
}