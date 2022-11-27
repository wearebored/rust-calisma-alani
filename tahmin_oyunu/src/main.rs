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
// use std::io;

// fn main() {
//     let mut sıcaklık =String::new();
//     println!("C sıcaklık değeri giriniz:");
//     io::stdin().read_line(&mut sıcaklık).expect("msg");
//     let sıcaklık:f64=sıcaklık.trim().parse().expect("msg");
//     let sıcaklık = sıcaklık*1.8+32.0;
//     println!("{sıcaklık}")



// }
// fn main() {
//     let mut sayı1=0;
//     let mut sayı2=1;
//     let mut kacdefa=String::new();
//     println!("fibonacci nin ilk kaç sayısı:");
//     io::stdin().read_line(&mut kacdefa).expect("msg");
//     let kacdefa:i64 = kacdefa.trim().parse().expect("msg");
//     println!("1={sayı2}");
//     for i in 2..kacdefa+1{
//         let sayı= sayı2;
//         sayı2+=sayı1;
//         sayı1=sayı;
//         println!("{i}={sayı2}")
//     }

// }



// fn main() {
//     let mut s= String::from("hello");
//     s.push_str(", world");
//     println!("{}",s)
// }
// fn main() {
//     let x=5;
//     let y=x;
//     let x =10;
//     println!("{x}");
//     println!("{y}")
// }
// fn main() {
//      let s1 = String::from("hello");
//      println!("{s1}");
//     let s2 = s1;
    
//     // println!("{s1}");
//     println!("{s2}")
// }
// fn main() {
//      let s1 = String::from("hello");
//     //  println!("{s1}");
//     let s2 = s1.clone();
    
//     println!("{s1}");
//     println!("{s2}")
// }


// fn main() {
//     let s1=String::from("hello");
//     let (s2,len)=karaktersay(s1);
//     println!("{s2},{len}");

    
// }

// fn karaktersay(s:String)->(String,usize) {
//     let len = s.len();
//     (s,len)
    
// }


// fn main() {
//     let s1=String::from("hello");
//     let len =karaktersay(&s1);
//     println!("{s1},{len}");

// }
// fn karaktersay(s1:&String)->usize {
//     s1.len()
// }

// fn main() {
//      let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
   
// }

// fn main() {
//     let kelime = String::from("keliadw  awdawd awd ");
//     let bosluk =boslukbul(&kelime);
//     // println!("{bosluk}")
// }

// fn boslukbul(s:&String)-> usize {
//     let ayrılmıs =s.as_bytes();
//     ayrılmıs

// }

// fn main() {
//     let kelime =String::from("123 123  321 efg  ");
//     let ayrılmıs=kelime.as_bytes();
  
//   for (i,&item) in ayrılmıs.iter().enumerate(){

//         if item == b' '{
//         //    println!("{i}");
//         // return i;
//         break
//         }
//         // println!("{item}");
//         // println!("{i}")
        
//     };
//     println!("{}",ayrılmıs.len());
// // println!("{a}")
//     // println!("{ayrılmıs}");


// }


// fn main() {
//     let s = String::from("hello");
//     let ilk = &s[0..2];
//     println!("{ilk}");
//     println!("{s}")

// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);
//     println!("{}",word)

//     // let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     // let word = first_word(&my_string_literal[0..6]);
//     // let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     // let word = first_word(my_string_literal);
// }

// fn main() {
//     let a = [1,2,3,4,5,6,7,8];
//     let b = &a[0..4];
//     let c =&[2,10];
//     for i in c{
//         println!("{i}");
//     }


// }