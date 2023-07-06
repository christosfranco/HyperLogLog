mod add_lib;

fn main() {
    use add_lib::internal::*;
    println!("{}",add(5,2));

    let value=    add_result(5,2);
    println!("{:?}",value);
}
