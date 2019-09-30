use libc::c_char;
use std::ffi::c_void;

#[link(name = "dl")]
extern "C" {
    #[allow(dead_code)]
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

/// Allows to overwrite functions which are defined within C.
///
/// Example (fakeroot):
/// ```rust,norun
/// use libc::uid_t;
/// ld_preload_function! {
///     unsafe fn getuid() -> uid_t => root_user_id {
///         0
///     }
/// }
/// ```
#[macro_export]
macro_rules! ld_preload_function {
    (unsafe fn $realname:ident($($paramname:ident : $paramtype:ty),*) -> $ret:ty => $customname:ident $block:block) => {
        #[allow(non_camel_case_types, dead_code)]
        pub struct $realname {
            _privatefield: (),
        }

        #[allow(non_camel_case_types, dead_code, non_upper_case_globals)]
        static $realname: $realname = $realname { _privatefield: () };

        impl $realname {
            fn get_inner(&self) -> unsafe extern "C" fn ($($paramname: $paramtype),*) -> $ret {
                use ::std::sync::{Once, ONCE_INIT};

                static mut REAL_PTR: *const u8 = 0 as *const u8;
                static mut ONCE: Once = ONCE_INIT;
                unsafe {
                    ONCE.call_once(|| {
                        let sym = $crate::dlsym(-1isize as *const c_void, concat!(stringify!($ealname), "\0").as_ptr() as *const c_char);
                        if sym.is_null() {
                            panic!("dlsym (ld_preload): Cannot find {}", stringify!($realname));
                        }
                        REAL_PTR = sym as *const u8;
                    });
                    ::std::mem::transmute(REAL_PTR)
                }
            }

            #[no_mangle]
            pub unsafe extern fn $realname($($paramname: $paramtype),*) -> $ret {
                ::std::panic::catch_unwind(|| $customname($($paramname),*)).ok()
                    .unwrap_or_else(|| $realname.get_inner()($($paramname),*))
            }
        }

        pub unsafe fn $customname($($paramname: $paramtype),*) -> $ret {
            $block
        }
    };
}

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
