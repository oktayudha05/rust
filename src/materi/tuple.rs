#[cfg(feature = "tuple")]
pub fn run(){
    // tuple
    let mut slice = (1, "ikan", true);
    println!("{slice:?}");
  
    let slice1 = slice.1;
    slice.1 = "yanto";
  
    println!("{}", slice1);
    println!("{}", slice.1);
    println!("{slice:?}");
  
}