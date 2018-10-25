use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("slice first {}", slice[0]);
    println!("slice length {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("{}", xs[0]);
    println!("{}", xs[1]);
    println!("length {}", ys.len());

    println!("array memory size in bytes {}", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&ys[1..2]);
}