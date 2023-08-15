fn main() {
    let my_name = "Pascal";
    greet(my_name.to_string());
}

fn greet(name: String){
    println!("Hello {}.", name);
}
