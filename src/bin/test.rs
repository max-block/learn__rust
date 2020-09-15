fn main() {
    let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let a = &arr[2..=99];
    println!("{:?}", a);
}
