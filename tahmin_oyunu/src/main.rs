// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// fn main() {
//     println!("Sayıyı tahmin edin! ");
//     let rasgele =rand::thread_rng().gen_range(1..=100);
//     // println!("rasgele sayı : {rasgele}");
//     loop{
//     println!("Bir sayı giriniz:");
//     let mut sayı = String::new();
//     io::stdin().read_line(&mut sayı).expect("failed to read line");
//     let sayı: u32= match sayı.trim().parse(){
//         Ok(num)=>num,
      
//         Err(_)=>continue,
//     };
//     println!("sectiğin sayı: {sayı}");
//     match sayı.cmp (&rasgele){
//         Ordering::Less=>println!("sayı küçük"),
//         Ordering::Greater=> println!("sayı büyük"),
//         Ordering::Equal=>{println!("kazandın");
//     break;
//     }
    
//     }
// }


// }

// fn main(){
//     let mut x = 5;
//     println!("x in değeri {x} tir");
//     x=22;
//     println!("x in yeni değeri {x} tir");
// }
// fn main(){
//     let x = 5;
//     let x =x+10;
//     {
//         let x =x *2;
//         println!{"x in son değeri {x} dir"}

//     }
//     println!{"x in değeri{x}"}
// }

// fn main(){
//     let boşluk = "      ";
//     let boşluk = boşluk.len();
//     println!("boşluk sayısı {boşluk}")
// }


// fn main(){
//     let sayi:u32="43".parse().expect("sayı bulunamadı");
//     println!("{sayi}")
// }
// fn main()  {
//        let say:&str="dadwdwadawda2143123124214dawdawdawd2q31rd123414c21vc4 2134 124 1241242134123daw";
//        println!("{}",say.len())
    
    
//     }

