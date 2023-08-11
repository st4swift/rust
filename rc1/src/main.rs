use std::fmt::Debug;

fn main() {
    let x = '中'; 
    println!("size of 中 is {}.", std::mem::size_of_val(&x));

    let y = {
        let x = 8;
        x + 6
    };
    println!("y is {}", y);

    assert_eq!(ret_unit_type(), ());

     report(78);

    let x = 80;
    println!("plus_or_minus is {}", plus_or_minus(x));
    let x = -80;
    println!("plus_or_minus is {}", plus_or_minus(x));
 
}


fn plus_or_minus(x: i32)->i32{
    if x>5 {
        return x-5
    }
    x+5
}

fn report<T:Debug>(item: T){
    println!("{:?}", item);
}



fn ret_unit_type(){
    let x = 1;
    let z = if x%2 == 1 {"odd"} else {"even"};
    println!("x is {}, z is {}.", x, z);
}




