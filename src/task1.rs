fn collatz_length(mut n: i32) -> i32 {
    let mut count = 1;
    let mut num = n;

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
        count += 1;
    }

    count
}

fn main() {
    let n = 6;
    let length = collatz_length(n);
    println!("The length of the Collatz sequence for {} is {}", n, length);
}