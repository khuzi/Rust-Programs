pub mod input{
    use std::io;
    pub fn read_inp(mut int: String)->i32{
    let  number: i32;
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");
    let _int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    number = _int;
    number
    }
}


pub mod input_f{
    use std::io;
    pub fn read_inp(mut int: String)->f64{
    let  number: f64;
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");
    let _int: f64 = int.trim().parse().unwrap();
    number = _int;
    number
    }
}



pub mod triangle{
    #[derive(Debug)]
  pub  struct triangle_prop {
        height: f64,
        base: f64,
    }
    impl triangle_prop{
    pub  fn new(height: f64 , base: f64)-> triangle_prop
    {
        triangle_prop{
            height,
            base,
        }

    }

  pub  fn area_tri(&self){
        let area: f64 = (self.height*self.base)*0.5;
        println!("Area of a Triangle with Height {} and Base {} = {} ",self.height,self.base,area);
    }
  }
}



