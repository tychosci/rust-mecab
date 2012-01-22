use std;

import option::{some, none};

export mecab_new, mecab_do, imecab;

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
    fn mecab_strerror(mecab: *mecab_t) -> str::sbuf;
    fn mecab_sparse_tostr(mecab: *mecab_t, input: str::sbuf) -> str::sbuf;

}

//-- FIXME ------------------------------------------------------
//
//   These are only *test* implementation.
//

iface imecab {
    fn strerror() -> str;
    fn check_it<V>(c: option::t<*V>);
    fn sparse_tostr(input: str) -> str;
}

impl of imecab for *mecab::mecab_t {

    fn strerror() -> str unsafe {
        let res = mecab::mecab_strerror(self);
        str::from_cstr(res)
    }

    fn check_it<V>(_c: option::t<*V>) {
        // do nothing
    }

    fn sparse_tostr(input: str) -> str unsafe {
        let res = str::as_buf(input) { |buf|
            mecab::mecab_sparse_tostr(self, buf)
        };
        str::from_cstr(res)
    }

}

impl <T: imecab, C> of imecab for {base: T, cleanup: C} {

    fn strerror() -> str unsafe {
        self.base.strerror()
    }

    fn check_it<V>(c: option::t<*V>) {
        alt c {
            some::<*V>(_) { /* do nothing */ }
            none { fail #fmt["Exception: %s", self.strerror()]; }
        }
    }

    fn sparse_tostr(input: str) -> str {
        self.base.sparse_tostr(input)
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

