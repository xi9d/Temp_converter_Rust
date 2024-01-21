use std::io;
fn main() {
    println!("Choose the mode: /n1. Celcius to Farahenheight./n2. Farahenheight to celcius");
    let mut temp = String::new();
    let mut mode = String::new();
    io::stdin()
    .read_line(&mut mode)
    .expect("No selected mode");

    println!("Now enter the temprature");

    io::stdin()
        .read_line(&mut temp)
        .expect("No selected mode");

    let mode: u32 = mode.trim().parse().expect("msg");
    let temp: f64 = temp.trim().parse().expect("msg");

    println!("The mode is:{mode}");
    
    println!("your before temp is {temp}");
    if mode == 1 {
        let x = celcius_to_fa(temp);
        println!("{x}");
    }else {
        fa_to_celcius(temp);
    }
    
}
fn celcius_to_fa(value: f64) -> f64{
     (value * 9.0/5.0)+ 32.0
   
}

fn fa_to_celcius(value: f64){
   let result = (value - 32.0) * 9.0/5.0;

    print!("Fa to celcius from function{result}");
}

// wha happens in a temprature converter,
/*
1. you get temprature either in celcius or in fh
2. you need to calculate or cal a function that does that
*/