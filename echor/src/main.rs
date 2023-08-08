use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("st <st@gmail.com>")
        .about("rust_echo")
        .get_matches();

    println!("{:?}", std::env::args());
    
}
