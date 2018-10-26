fn main() {
    let mut count = 10;

    loop {
        count -= 1;

        if count <= 0 {
            break;
        }

        if count % 2 == 0 {
            continue;
        }

        println!("Counter {}", count);
    }

    count = 0;

    let res = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    assert_eq!(res, 20);

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}