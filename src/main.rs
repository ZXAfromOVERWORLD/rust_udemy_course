/* #[derive(Debug)]
struct Man{
    name: String,
    age: i32,
    email: String,
    country: String,
}

fn _area(user: &Man) -> &String{
    &user.country
}

fn main() {
    println!("Hello, world!");
    let mut x = 23;
    println!("{}",x);
    x += 1;
    println!("{}",x);

    let man1 = Man{
        name:String::from("Siddham"),
        age:32,
        email:String::from("siddhamnegi@gmail.com"),
        country:String::from("INDIA"),
    };
    println!("{}", man1.nam());
}

impl Man {
    fn nam(&self) -> &String{
        &self.name
    }
}*/

/*fn main(){
    let mut x = [0;5];
    let u:[i32;9]=[2,3,45,6,1,34,64,21,67];
    println!("{:?}",x);
    for c in x.iter_mut(){
        *c += 1;
    }
    println!("{:?}",x);
    for t in u.into_iter().rev().step_by(2){
        println!("{}",t);
    }
    for (index,j) in x.iter().enumerate(){
        println!("The number at pos {} is {}",index,j);
    }
    println!("{:?}",u.last());
    println!("{:?}",u.split_at(4));
    let sum:i32 = x.iter().sum();
    println!("{}",sum);
}*/
/*mod garden;

use rand::Rng;
use garden::first::number;
fn main(){
    number();
    let sec=rand::thread_rng().gen_range(1..10);
    println!("{:?}",sec);
    let mut map = HashMap::new();
    map.insert(sec,"Siddham");
    println!("{:?}",map);

}*/
/*use std::collections::HashMap;
fn main(){
    let s = "Rohan Agarwal";
    let m = &s[0..5];
    let m2 = &s[6..13];
    println!("{}", m);
    println!("{}",m2);
    for c in s.chars(){
        println!("{}",c);
    }
    let mut map = HashMap::new();
    let mut money = 100;
    for i in 1..10{
        map.insert(i,money);
        money+=100;
    }
    println!("{map:?}");
    let a: [u8 ; 5] = [10,20,30,40,50];
    if let Some(&ans)=a.get(2){
        println!("{}",ans);
    }
}*/
/*#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

use std::net::IpAddr;
fn main(){
    //panic!("Burn and Rot in help you sunvabisch");
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP");
    println!("{}",home);
    let namo = Guess::new(23);
    println!("{namo:?}");
    println!("{}",namo.value());
    let nainshi = String::from("Moti");
    println!("Nainshi {}",nainshi);
}*/
pub fn bubble_sort<T:PartialOrd>(v: &mut[T]){
    for _j in 0..v.len(){
        for i in 0..(v.len()-1){
            if v[i] > v[i+1]{
                v.swap(i, i+1)
            }
        }
    }
} 


fn main(){
    println!("Moti Nainshi");
    let mut l = vec![4,23,7,12,9,72,53];

}



