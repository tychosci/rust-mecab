use std;

export mecab_new, imecab;

//-- FIXME ------------------------------------------------------
//
//   This would be failed if users installed 'mecab'
//   with `--prefix=...` option on `./configure`.
//
#[link_args = "-Wl,-rpath,/usr/local/lib"]
#[link_name = "mecab"]
#[abi = "cdecl"]
native mod mecab {

    // FIXME: add more types that needed to use in this binding.
    type mecab_t;
    type mecab_node_t;
    type mecab_dictionary_info_t;

    // FIXME: add more functions.
    fn mecab_new(argc: ctypes::c_int, argv: *str::sbuf) -> *mecab_t;
    fn mecab_destroy(mecab: *mecab_t);
    fn mecab_do(argc: ctypes::c_int, argv: *str::sbuf) -> ctypes::c_int;

}

//-- FIXME ------------------------------------------------------
//
//   These are only *test* implementation.
//

iface imecab {
    fn print();
}

impl of imecab for *mecab::mecab_t {
    fn print() {
        std::io::println("woof");
    }
}

impl <T: imecab, C> of imecab for {base: T, cleanup: C} {
    fn print() {
        self.base.print();
        std::io::println("meow");
    }
}

//-- FIXME ------------------------------------------------------
//
//   write documentation.
//

resource wrapped_mecab(m: *mecab::mecab_t) {
    mecab::mecab_destroy(m);
}

fn mecab_new(argc: uint, args: [str]) -> imecab unsafe {
    let argc = argc as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let m = mecab::mecab_new(argc, vec::unsafe::to_ptr(argv));
    {base: m, cleanup: wrapped_mecab(m)} as imecab
}

fn mecab_do(argc: uint, args: [str]) -> int unsafe {
    let argc = argc as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let res = mecab::mecab_do(argc, vec::unsafe::to_ptr(argv));
    res as int
}
