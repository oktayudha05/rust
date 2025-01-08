mod materi{
    pub mod variable;
    pub mod operator;
    pub mod condition;
    pub mod perulangan;
    pub mod array;
    pub mod slice;
}
fn main() {
    #[cfg(feature = "variable")]
    materi::variable::run();
    
    #[cfg(feature = "operator")]
    materi::operator::run();

    #[cfg(feature = "condition")]
    materi::condition::run();

    #[cfg(feature = "perulangan")]
    materi::perulangan::run();

    #[cfg(feature = "array")]
    materi::array::run();

    #[cfg(feature = "slice")]
    materi::slice::run();
}
