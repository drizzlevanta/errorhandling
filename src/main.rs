fn main() {
    propagate_errors::use_errors();
    unwrap_expect::expect();
    openfile::open_file();
}

mod openfile;
mod propagate_errors;
mod unwrap_expect;
