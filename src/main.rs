use std::io;
use rc_lib::{input,input_f,triangle};
fn main() {
//====================================================Answer NO.1===========================================================
    println!("Enter the Radius of a circle:");
    let mut rad = String::new();
    io::stdin().read_line(&mut rad);
    let rad: f64 = rad.trim().parse().unwrap();
    println!("The Area of a Circle is={}",cal_area(rad));

//====================================================Answer NO.2===========================================================
 println!("Enter the number:");
     let  num = String::new();
   let num =  input::read_inp(num);
    pos_neg(num);

//====================================================Answer NO.3===========================================================
println!("Enter Numerator:");
    let  n = String::new();
    let n = input::read_inp(n);
    println!("Enter Denominator:");
    let  d = String::new();
    let d = input::read_inp(d);
    divis_check(n,d);




//====================================================Answer NO.4===========================================================
  println!("Enter the Radius of a sphere:");
    let mut radi = String::new();
    io::stdin().read_line(&mut radi);
    let radi: f64 = radi.trim().parse().unwrap();
    println!("The Volume of a Sphere is={}",cal_volume(radi));

//====================================================Answer NO.5===========================================================
println!("Enter String:");
let mut word = String::new();
io::stdin().read_line(&mut word);
println!("How many copies of String you need:");
let  num1 = String::new();
let num1 = input::read_inp(num1);
str_inp(num1, word);

//====================================================Answer NO.6===========================================================
println!("Enter Any Number:");
let num2 = String::new();
let num2 = input::read_inp(num2);
even_odd(num2);

//====================================================Answer NO.7===========================================================
println!("Enter magnitude of Triangle Height:");
let height = String::new();
let height = input_f::read_inp(height);
println!("Enter magnitude of Triangle Base:");
let base = String::new();
let base = input_f::read_inp(base);
let triangle = triangle::triangle_prop::new(height,base);
triangle.area_tri();

//====================================================Answer NO.8===========================================================
    let mut num3 = String::new();
let mut vec: Vec<i32> = Vec::new();
println!("Enter Co-ordinates x1 , x2 , y1 , y2 respectively:");
io::stdin().read_line(&mut num3);
for i in num3.split_whitespace(){
    let value: i32 = i.parse().unwrap();
    vec.push(value);
}
let d1: i32 = (vec[1] - vec[0]).pow(2);
let d2: i32 = (vec[3] - vec[2]).pow(2);
 let distance: f32 = (d1 as f32 + d2 as f32).powf(0.5);
 println!("Distance Between ({},{}) ({},{}) is {}",vec[0],vec[1],vec[2],vec[3],distance);
}

































//======================================Function of ANS.1=================================================================
fn cal_area(r: f64)->f64{
    const PIE: f64 = 3.1425;
    let  area: f64 = PIE*r*r;
    area

}
//======================================Function of ANS.2=================================================================
fn pos_neg(n: i32){
    if n > 0{
        println!("Entered Number is Positive");
    }
    else if n < 0{
        println!("Entered Number is Negative");
    }
    else {
        println!("ZERO Entered");
    }
}

//======================================Function of ANS.3=================================================================
fn divis_check(n: i32,d: i32){
    if n%d==0{
        println!("{} is completely divisible by {}",n,d);
    }
     if n%d!=0{
        println!("{} is not completely divisible by {}",n,d);
    }
}

//========================================================Function of ANS.4=================================================================
fn cal_volume(r: f64)->f64{
    const PIE: f64 = 3.1425;
    let  volume: f64 = (1.33333333333)*PIE*r*r*r;
    volume
}

//=========================================================Function of ANS.5=================================================================
fn str_inp(num: i32 , word: String){
    print!("{} copies of {}",num,word);
    for _i in 1..=num{
        print!("{}",word);
    }
}
//=========================================================Function of ANS.6=================================================================
fn even_odd(num: i32){
    if num%2==0{
        println!("{} is Even",num);
    }
    else {
        println!("{} is Odd",num);
    }
}
