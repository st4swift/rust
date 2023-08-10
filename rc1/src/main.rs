
fn main() {
    let x = '中'; 
    println!("size of 中 is {}.", std::mem::size_of_val(&x));

    let y = {
        let x = 8;
        x + 6
    };
    println!("y is {}", y);

    assert_ne!((), ret_unit_type())

}

fn ret_unit_type(){
    let x = 1;
    let z = if x%2 == 1 {"odd"} else {"even"};
    println!("x is {}, z is {}.", x, z);
}




