#[cfg(feature = "array")]
pub fn run(){
  // bikin array typr inference
  let mut numbers = [1, 4, 14, 41];
  println!("array numbers {numbers:?}");
  numbers[1] = 12;
  println!("array numbers stlh di ubah {numbers:?}");
  println!("array dengan index ke 2 adalah {}", numbers[2]);

  // manifest typing
  let data_boolean: [bool; 2] = [true, false];
  println!("{data_boolean:?}");

  // notasi [T; N]
  let data_numerik: [i8; 8] = [14; 8];
  println!("{data_numerik:?}");
  
  // masih sama [T; N]
  let data_numerik2 = [41; 3];
  println!("{data_numerik2:?}");
  let pjg_arr = data_numerik2.len();
  println!("panjang array data_numerik \t : {pjg_arr}");

  //iterasi array dengan for in
  let buahs: [&str; 3] = ["jeruk", "apel", "cubung"];
  let mut i = 0;
  for buah in buahs {
    i+=1;
    println!("buah ke {i} \t -{buah}");
  }

  // iterasi array pake while
  let mut i = 0;
  while i < buahs.len() {
    i+=1;
    println!("iterasi pake while ke {i}");
  }

  // iterasi array pake loop
  let mut i = 0;
  loop {
    i+=1;
    if i > buahs.len() {
      break;
    }
    println!("iterasi pake loop ke {i}");
  }

  // iterasi pake for in dan tuple
  for (i, buah) in buahs.iter().enumerate(){
    println!("buah ke {i} adalah {buah}");
  }

  // nested array
  let nest = [
    ["agus", "sakit", "buk"],
    ["sucipto", "uni", "bakwan"],
    ["yanto", "kopling", "kak gem"],
  ];

  for baris in nest { // kepemilikan nest di pindah ke for.. jika tidak ingin brrti pakai &nest
    for el in baris {
      print!("{el} ");
    }
    println!();
  }
}
