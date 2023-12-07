// using mut string to edit strings, adding char and str to the present string.

pub fn run(){
    let mut s: String = String::from("");
    s.push_str("hello world");
    s.push('!');
    s +="!";
    assert_eq!(s,"hello world!!");
    println!("{}",s);
    println!("Success!");
    let p:String = String::from("I like dogs");
    let p1= p.replace("dogs","cats");
    assert_eq!(p,"I like cats");

}