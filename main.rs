fn main() {

    println! ("{}", print_hello_world())

}

fn print_hello_world() -> &'static str {

    let x = "Hello World";
    x
}
