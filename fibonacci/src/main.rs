use text_io::read;


fn main() {
    println!("Enter length of sequence:");

    let length: usize = read!();

    let mut sequence: Vec<usize> = Vec::new();
    sequence.push(1);
    sequence.push(1);

    println!("Sequence start:");

    for i in 0..length {
        if i > 1 {
            let n = &sequence[i-1]+&sequence[i-2];
            sequence.push(n);
        }

        println!("{}", &sequence[i]);

        if i == length - 1 {
            println!("Golden ratio approximation: {}", (*&sequence[i] as f64 / *&sequence[i-1] as f64));
        }
    }

    
}
