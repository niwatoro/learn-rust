use std::io;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let three_threes = [3; 3]; // [3, 3, 3]

    let first = arr[0];
    let second = arr[1];

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index must be a number");

    let element = arr[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
