use std::io;

fn main() {
    
    let mut word = String::new(); 
    println!("nhap 1 tu:", );
    io::stdin().read_line(&mut word);
    println!("Tu vua nhap la:{}", word);
}
