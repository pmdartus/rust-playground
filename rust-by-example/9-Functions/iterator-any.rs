fn main() {
    let vec1 = vec![1, 2, 3];

    println!("has 2 {}", vec1.iter().any(|&x| x == 2));
    println!("has 2 {}", vec1.into_iter().any(|x| x == 2));
}