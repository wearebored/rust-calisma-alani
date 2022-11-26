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
// fn main(){
//     let say:i128=123133333324;
//     println!("{}",say);
//     print!("dawdwa")
// }

// fn main(){
//     let x:f32=1.4;
//     let y :f32=9.5;
//     print!("{x}\n");
//     print!("{y}")

// }
// fn main(){
//     let a = 49-33;
//     let b:f64 = 23.8-33.0;
//     print!("{b}");
//     print!("{a}")
// }
// fn main() {
//     let sayilar:(f64,i64,i8)= (12.2,4231,2);
//     let (c,v,b)=sayilar;
//     print!("{},{},{}\n",c,v,b);
//     print!("{},{},{}",sayilar.0,sayilar.1,sayilar.2);
// }
// fn main() {
    
//     let a =[1,2,3,4,5];
//     print!("{}",a[1]);
//     let sayilar=12;
    
    
// }
// use std::io;

// fn main() {
//     let a:[i32;6]=[1,2,3,4,5,6];
//     println!("bir sayı giriniz");
//     let mut index:String=String::new();
//     io::stdin().read_line(&mut index)
//     .expect("msgdwa");
//     let index:usize=index.trim().parse().expect("msgdwa");
//     let eleman:i32 = a[index];
//     print!("{}",eleman);
    
    
// }

// fn degerler() {
//     println!("değerler");
    
// // }
// fn main() {
//     println!("sayılar");
//     // degerler();
//     deneme(34,'a');
// }
// fn deneme(a:i32,b:char){
// println!("{a}");
// println!("{b}");
// println!("{c}");
// }
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// // }
// use std::io;
// fn main(){
//     println!("bir sayı giriniz");
//     let mut a = String::new();
//     io::stdin().read_line(&mut a).expect("msg");
//     let a:usize=a.trim().parse().expect("msg");
//     if a>10{
//         println!("{a}sayısı 10 dan büyük")
//     } else if a==9{
//         println!("dawd")
//     }   else{
//         println!("{a}sayısı 10 dan küçük")
//     }
// }

// fn main() {
//     let mut counter = 0;
//     let sayi =loop{
//         counter+=1;
//         if counter==10{
            
//             break counter*20 ;
//         }
//     };
//     println!("{sayi}");
//     println!("{counter}")

// }

// fn main() {
//     for number in (1..10) {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }