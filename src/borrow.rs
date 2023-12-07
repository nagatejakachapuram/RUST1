// Using borrow functions with mut strings
pub fn run(){
   let mut s = String::from ("hello");
   let r1=&s; // no prblm
   let r2=&s; // no prblm 
   println!("{} and {}",r1,r2); // r1, r2 will not be used after printing 
   let r3 = &mut s;
   println!("{}",r3);
}

// Borror rules
// 1. either have unlimited immutable or 1 mutable
// 2. references must be valid