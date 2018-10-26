fn is_odd(value: u32) -> bool {
    value % 2 == 1
}

fn main() {
    let upper = 1000;

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("Imperative approach {}", acc);

    let sum = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);

    println!("Functional approach {}", sum);
}