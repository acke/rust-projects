fn main() {

    print_hello_world()

}

fn print_hello_world() {

    let pair = ("Hello World", 10);
    
    println!("{}", pair.0);
    println!("{}", pair.1);
}
