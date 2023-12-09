// String ownerships and using &str literals

pub fn run(){
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); // String -> &str
    assert_eq!("hello,world!",s3);
    println!("{}",s3);
}