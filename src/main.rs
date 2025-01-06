mod variable;
mod operator;
mod condition;
fn main() {
    #[cfg(feature = "variable")]
    variable::run();
    
    #[cfg(feature = "operator")]
    operator::run();

    #[cfg(feature = "condition")]
    condition::run();
}
