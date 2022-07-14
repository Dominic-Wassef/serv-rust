fn add(_num_one: i32, _num_two: i32) -> i32 {
    return _num_one + _num_two;
}
fn main() {
    let mut total = add(10, 15);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify for free shipping");
        free_shipping = true;
    } else if total > 20 {
        println!("If you add ore item, you can qualify for free shipping")
    } else {
        println!("No free shipping")
    }

    total = match free_shipping {
        true => total + 0,
        false => total + 5,
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found!"),
    }

    println!("Total: {:?}", total);

    let items = [1, 2, 3, 4, 5];
    println!("{:?}", items);
}
