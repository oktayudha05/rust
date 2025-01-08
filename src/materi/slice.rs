#[cfg(feature = "slice")]
pub fn run(){
  // slice dari array
  let arr = [2, 4, 78, 14, 61]; // ada di stack
  let slc = &arr[1..4]; // data merujuk ke arr yang ada di stack.. bukan ngopi (Pinjem secara read-only.)
  println!("{slc:?}");

  // slice mutable reference
  let mut arr = [8, 7, 5, 12, 14];
  let slc = &mut arr[2..]; // (Pinjem secara mutable, jadi bisa diubah.)
  slc[1] = 14;
  println!("{slc:?}");

  // for in pada slice
  let scores = [7, 8, 9];
  for score in &scores[..]{
    println!("{score:?}");
  }

  // for in pada mutable slice
  let mut scores = [7, 8, 9];
  let slice = &mut scores[..];
  for score in &mut slice[..]{
    *score+=1;
  }
  println!("score : {scores:?}");
}