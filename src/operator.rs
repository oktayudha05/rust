#[cfg(feature = "operator")]
pub fn run(){
    let (num1, num2) = (12, 4);
    println!("{}", num1 + num2);
    
    let num3 = num1 * num2;
    println!("num3 {}", num3);
}