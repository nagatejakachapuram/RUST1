// done with creation of cargo build with mod run()

// 1
 pub fn run(){
    let x:i32 = 5;
     //fill in the blank
    let p: &i32 = &x;
    println!("The address of x is {:p}",p);


 }
// 2 
pub fn run(){
    let x: i32 = 5;
    let y : &i32 = &x;
    assert_eq!(5,*y);
    println!("Success!");
}
// 3
pub fn main(){
    let mut s : String = String::from("hello");
    let p = &mut s;
    p.push_str("world");
    println!("success!");
} 
