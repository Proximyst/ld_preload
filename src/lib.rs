#[macro_export]
macro_rules! ld_preload_init {
    ($block:block) => {
        #[no_mangle]
        #[link_section = ".init_array"]
        pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_initialise_fn;

        extern "C" fn ld_preload_initialise_fn() {
            $block
        }
    };
}

#[macro_export]
macro_rules! ld_preload_deinit {
    ($block:block) => {
        #[no_mangle]
        #[link_section = ".fini_array"]
        pub static LD_PRELOAD_DEINITIALISE_RUST: extern "C" fn() = self::ld_preload_deinitialise_fn;

        extern "C" fn ld_preload_deinitialise_fn() {
            $block
        }
    };
}
