#[cfg(feature = "perulangan")]
pub fn run(){
    use std::time::Instant;

    // while
    println!("loop while");
    let max = 10;
    let mut i = 0;
    
    let start = Instant::now();
    while i < max {
      i+=1;
      println!("nilai ke {}", i);
    }
    let durasi = start.elapsed();
    println!("durasi \t : {:?}", durasi);

    // loop
    println!("loop");
    let max_loop = 20;
    let mut i_loop = 0;
    loop {
      println!("ini pake loop ke {}", i_loop);
      i_loop += 1;
      if i_loop > max_loop {
        break;
      }
    }

    // loop break
    println!("loop break");
    let mut i = 0;
    let max = 5;
    loop {
      let mut j = max;
      let max_inner = i;

      loop {
        print!("* ");
        j-=1;
        if j < max_inner {
          break;
        }
      }
      println!();
      i+=1;
      if i > max{
        break;
      }
    }

    // loop continue
    println!("loop continue");
    let mut i = 0;
    let max = 10;
    loop {
      i+=1;
      if i % 2 == 1{
        continue;
      }
      println!("{i}");
      if i >= max{
        break
      }
    }

    // lable loop
    println!("label loop");
    let mut i = 0;
    let max = 5;
    'mainloop: loop{
      i+=1;
      let mut j = 0;
      loop{
        if i >= max {
          break 'mainloop;
        }
        j+=1;
        if j > i{
          break;
        }
        print!("{i} ");
      }
      println!()
    }

    // returning from loop
    println!("returning from loop");
    let mut counter = 0;
    let result = loop {
      counter += 1;
      if counter == 10{
        break counter*2;
      }
    };
    println!("hasilnya adlh \t : {}", result);

    // for in
    println!("for in");
    for i in 1..=5{
      println!("{i}");
    }

    // label for in
    println!("label for in");
    'perulangan:for i in 0..10{
      if i > 5{
        println!("loop di berhentikan pada iterasi ke {}", i);
        break 'perulangan;
      }
      println!("{i}");
    }

    // looping pada array
    println!("loop pada array");
    let names = ["panjul", "bokol", "agus", "yanto kopling", "saus tomat"];
    for name in names{
      println!("\t - {}", name);
    }
}