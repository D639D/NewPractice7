mod folder1;
mod folder2;

fn main() {
    folder1::test1::world1(String::from("Egor1"));
    folder2::test2::world2(String::from("Egor2"));
}