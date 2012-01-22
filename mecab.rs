use std;

import option::{some, none};

export mecab_new, mecab_do, mecab;

//-- FIXME ------------------------------------------------------
//
//   This would be failed if users installed 'mecab'
//   with `--prefix=...` option on `./configure`.
//
#[link_args = "-Wl,-rpath,/usr/local/lib"]
#[link_name = "mecab"]
#[abi = "cdecl"]
native mod _mecab {

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

iface mecab {
    fn strerror() -> str;
    fn sparse_tostr(input: str) -> option::t<str>;
}

impl of mecab for *_mecab::mecab_t {

    fn strerror() -> str unsafe {
        let res = _mecab::mecab_strerror(self);
        str::from_cstr(res)
    }

    fn sparse_tostr(input: str) -> option::t<str> unsafe {
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tostr(self, buf)
        };

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

}

impl <T: mecab, C> of mecab for {base: T, cleanup: C} {

    fn strerror() -> str unsafe {
        self.base.strerror()
    }

    fn sparse_tostr(input: str) -> option::t<str> {
        self.base.sparse_tostr(input)
    }

}

//-- FIXME ------------------------------------------------------
//
//   write documentation.
//

resource wrapped_mecab(m: *_mecab::mecab_t) {
    _mecab::mecab_destroy(m);
}

fn mecab_new(argc: uint, args: [str]) -> mecab unsafe {
    let argc = argc as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let m = _mecab::mecab_new(argc, vec::unsafe::to_ptr(argv));
    {base: m, cleanup: wrapped_mecab(m)} as mecab
}

fn mecab_do(argc: uint, args: [str]) -> int unsafe {
    let argc = argc as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let res = _mecab::mecab_do(argc, vec::unsafe::to_ptr(argv));
    res as int
}

