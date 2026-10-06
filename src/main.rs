use Practice_7::math_utils;

fn main() {
    let one = math_utils::max_of_two(1.0, 2.0);
    let two = math_utils::min_of_two(1.0, 2.0);
    println!("{}", one);
    println!("{}", two);
}
