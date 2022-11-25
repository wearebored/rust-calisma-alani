use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Sayıyı tahmin edin! ");
    let rasgele =rand::thread_rng().gen_range(1..=100);
    // println!("rasgele sayı : {rasgele}");
    loop{
    println!("Bir sayı giriniz:");
    let mut sayı = String::new();
    io::stdin().read_line(&mut sayı).expect("failed to read line");
    let sayı: u32= match sayı.trim().parse(){
        Ok(num)=>num,
      
        Err(_)=>continue,
    };
    println!("sectiğin sayı: {sayı}");
    match sayı.cmp (&rasgele){
        Ordering::Less=>println!("sayı küçük"),
        Ordering::Greater=> println!("sayı büyük"),
        Ordering::Equal=>{println!("kazandın");
    break;
    }
    
    }
}


}
