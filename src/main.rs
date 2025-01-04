use std::i8;

fn main() {
    let nama = "rio";
    println!("halo nama saya {}", nama);

    let (var1, mut var2, var3): (i8, i8, i8) = (64, 12, 8);
    let var4 = 18i8;
    println!("var1 {}, var2 {}, var3 {}", var1, var2, var3);
    var2 = 14;
    println!("var 2 diganti jadi {}", var2);
    println!("var 4 {}", var4);

    let i8_min = i8::MIN;
    let i32_min = u32::MAX;
    println!("i8 min = {i8_min}");
    println!("u32 min = {i32_min}");

    let floating_point1 = 3.345353;
    println!("fp1 = {:.3}", floating_point1)
}
