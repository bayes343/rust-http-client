use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{:?}", &args[1..]);
    } else {
        println!("You must enter a URI.")
    }
    // let stream = TcpStream::connect("api.github.com/repos/Fyord/Fyord/contributors:80");
}
