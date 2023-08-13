
fn main() {
    println!("{}", dangle());
}

fn dangle()->  String {
    let s = String::from("dangling references!");
    s
}

