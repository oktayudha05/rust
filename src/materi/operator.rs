#[cfg(feature = "operator")]
pub fn run(){
    let (num1, num2) = (12, 4);
    println!("{}", num1 + num2);

    let num3 = num1 * num2;
    println!("num3 {}", num3);

    let perb1 = num1 < num2;
    println!("perbandingan 1 {}", perb1);

    // negasi operator
    let (neg1, neg2) = (12, -12);
    let hasil_negasi1 = -neg1 == neg2;
    let hasil_negasi2 = !(neg1 == neg2);
    println!("hasil negasi 1 \t: {hasil_negasi1}");
    println!("hasil negasi 2 \t: {hasil_negasi2}")
}