mod variable;
mod operator;
mod condition;
mod perulangan;
fn main() {
    #[cfg(feature = "variable")]
    variable::run();
    
    #[cfg(feature = "operator")]
    operator::run();

    #[cfg(feature = "condition")]
    condition::run();

    #[cfg(feature = "perulangan")]
    perulangan::run();
}
