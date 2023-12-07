pub fn run(){
    // finding max size
    println!("MAX i32; {}",std::i32::MAX);
    println!("MAX i64; {}",std::i64::MAX);
    // Boolean
    let is_active: bool =true;
    // get boolean from expression
    let is_greater: bool = 10<5;
    println!("{:?}",(is_active, is_greater));

}