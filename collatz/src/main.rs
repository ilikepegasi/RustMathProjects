use text_io::read;

fn main() {
    println!("Enter initial value");
    let mut n: u64 = read!();
    let mut i: u64 = 0;
    if n <= 1 {
        println!("Enter an integer above 1")
    } else {
        while n != 1 {
            if n % 2 == 1 {
                n = n*3 + 1;
            } else {
                n = n / 2;
            }
            println!("{}", n);
            i = i + 1;
        }
        println!("The conjecture took {} iterations to finish", i);
    }
}
