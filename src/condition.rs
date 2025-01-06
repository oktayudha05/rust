#[cfg(feature = "condition")]
pub fn run(){
    let number = 5;

    if number <= 10 {
      println!("number lebih kecil dari 10");
    } else if number > 10 && number <= 100 {
      println!("number sedang bro");
    } else {
      println!("number gede bnr broo");
    }

    // jika gapake let if
    let apakah_benar: bool;
    let angka = 9;
    if angka > 5 {
      apakah_benar = true;
    } else {
      apakah_benar = false;
    }
    println!("nilai non let if adalah {}", apakah_benar);

    // kalo pake let if
    let angka2 = 9;
    let hasil = if angka2 > 5{
      true
    } else{
      false
    };
    println!("nilai pake let if adalah {}", hasil);
}