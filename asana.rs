use std::env;


fn main(){
    let version = "1.0.0";
    let args: Vec<_> = env::args().collect();


    if args[1] == "version" {
        println!("{}", version);
    }
    else if args[1] == "show" {
        println!("show what?");
    }
    else {
        println!("{}", args[1]);
    }
}
