pub fn map() {
    let v1: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];
    let v2: Vec<u32> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v2); // [2, 3, 4, 5, 6, 7, 8]
    println!("{:?}", v1); // v1 is still alive
}

pub fn filter() {
    let v1: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];
    let v2: Vec<_> = v1.iter().filter(|x| *x % 2 == 0).collect();

    println!("{:?}", v2); // [2, 3, 4, 5, 6, 7, 8]
                          // println!("{:?}", v1); // v1 is still alive
}

fn main() {
    // filter();
    map();
}
