use std::io;
fn main() {
    println!("Sayıyı tahmin edin! ");
    println!("Bir sayı giriniz:");
    let mut sayı = String::new();
    io::stdin().read_line(&mut sayı).expect("failed to read line");
    println!("sectiğin sayı: {sayı}");



}
