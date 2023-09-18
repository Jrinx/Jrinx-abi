macro_rules! decl_sysno {
    ($name:ident = $num:literal) => {
        pub const $name: usize = $num;
    };
    ($( $name:ident = $num:literal, )*) => {
        $( decl_sysno!($name = $num); )*
    }
}

decl_sysno! {
    SYS_EXIT = 0,
    SYS_READ = 1,
    SYS_WRITE = 2,
}
