#![allow(dead_code)]

#[derive(Debug)]
struct Data {
    name: String,
    value: i32,
}

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

pub fn filter_struct() {
    let data = vec![
        Data {
            name: "n1".to_string(),
            value: 10,
        },
        Data {
            name: "n2".to_string(),
            value: 20,
        },
        Data {
            name: "n3".to_string(),
            value: 30,
        },
    ];

    let data: Vec<_> = data.into_iter().filter(|x| x.value >= 20).collect();
    println!("{:#?}", data);
}

fn main() {
    // filter();
    // map();
    filter_struct()
}
