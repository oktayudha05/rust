mod variable;
mod operator;
fn main() {
    #[cfg(feature = "variable")]
    variable::run();
    
    #[cfg(feature = "operator")]
    operator::run()
}
