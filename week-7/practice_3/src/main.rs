fn value(n:Option<&char>) 
{
    println!("Element of vector {:?}",n);

}

fn main() {

    let v = vec!['R','U','S','T','A','C','I','A','N' ];

    Let mut input1 = String::new();
    println!("\nEnter an index value btw (0 -8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");


    let index:usixe = input1.trim().parse().expect("Invalid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
}