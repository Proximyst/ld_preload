#![no_std]

/// Creates a function with C's `__attribute__((constructor))` applied to it.
///
/// Under the hood it's just a section with `.init_array` and `.init_array_end` ASM sections.
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

/// Creates a function with C's `__attribute__((destructor))` applied to it.
///
/// Under the hood it's just a section with `.fini_array` and `.fini_array_end` ASM sections.
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
