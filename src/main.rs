use std::io; 
use std::io::Write;
fn main() {
    print!("Enter in a temperature in Fahrenheight: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();  

        io::stdin() 
        .read_line(&mut line)
        .expect("Failed to read line");

    let input: f32 = line
        .trim()
        .parse::<f32>()
        .expect("Not a valid number"); 
    
   let finalresult = f_to_c(input);  
   println!("The temperature in Celcius is {}", finalresult);

}
// F to C formula: 
// (F - 32) * 0.5556
fn f_to_c(input: f32)-> f32 { 
   let result : f32;  
   result = (input - 32.0) * 0.5556; 
   return result; 
}
