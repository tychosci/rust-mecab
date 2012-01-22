use std;

// -*- rust -*-
import option::{some, none};

export mecab_new, mecab_do;
export mecab;
// export mecab_dictionary_info;

/*

FIXME:

 This `link_args` would be failed if users installed 'mecab'
 with `--prefix=...` option on `./configure`.

*/

#[link_args = "-Wl,-rpath,/usr/local/lib"]
#[link_name = "mecab"]
#[abi = "cdecl"]
native mod _mecab {

    // FIXME: add more types that needed to use in this binding.
    type mecab_t;
    type mecab_node_t;
    type mecab_dictionary_info_t;

    // FIXME: add more functions.
    fn mecab_new(argc: ctypes::c_int, argv: *str::sbuf)
        -> *mecab_t;

    fn mecab_destroy(mecab: *mecab_t);

    fn mecab_strerror(mecab: *mecab_t)
        -> str::sbuf;

    fn mecab_do(argc: ctypes::c_int, argv: *str::sbuf)
        -> ctypes::c_int;

    fn mecab_sparse_tostr(mecab: *mecab_t, input: str::sbuf)
        -> str::sbuf;

    fn mecab_dictionary_info(mecab: *mecab_t)
        -> *::mecab_dictionary_info_t;

}

type mecab_dictionary_info_t =
    { filename: str::sbuf
    , charset:  str::sbuf
    , size:     ctypes::c_uint
    , type:     ctypes::c_int
    , lsize:    ctypes::c_uint
    , rsize:    ctypes::c_uint
    , version:  u16
    , next:     *_mecab::mecab_dictionary_info_t
    };

iface mecab_dictionary_info {
    fn bump();

    fn is_end() -> bool;

    fn get_filename() -> str;
    fn get_charset()  -> str;

    fn get_size()    -> uint;
    fn get_type()    ->  int;
    fn get_lsize()   -> uint;
    fn get_rsize()   -> uint;
    fn get_version() -> uint;
}

impl of mecab_dictionary_info for *mecab_dictionary_info_t {

    fn bump() unsafe {
    }

    fn is_end() -> bool unsafe {
        if self == ptr::null() { true } else { false }
    }

    fn get_filename() -> str unsafe {
        let buf = (*self).filename;
        str::from_cstr(buf)
    }

    fn get_charset() -> str unsafe {
        let buf = (*self).charset;
        str::from_cstr(buf)
    }

    fn get_size()    -> uint unsafe { (*self).size    as uint }
    fn get_type()    ->  int unsafe { (*self).type    as  int }
    fn get_lsize()   -> uint unsafe { (*self).lsize   as uint }
    fn get_rsize()   -> uint unsafe { (*self).rsize   as uint }
    fn get_version() -> uint unsafe { (*self).version as uint }

}

impl of mecab_dictionary_info for {mutable base: *mecab_dictionary_info_t} {

    fn bump() unsafe {
        self.base = (*self.base).next as *mecab_dictionary_info_t;
    }

    fn is_end() -> bool { self.base.is_end() }

    fn get_filename() -> str unsafe { self.base.get_filename() }

    fn get_charset()  -> str unsafe { self.base.get_charset() }

    fn get_size()     -> uint unsafe { self.base.get_size() }
    fn get_type()     ->  int unsafe { self.base.get_type() }
    fn get_lsize()    -> uint unsafe { self.base.get_lsize() }
    fn get_rsize()    -> uint unsafe { self.base.get_rsize() }
    fn get_version()  -> uint unsafe { self.base.get_version() }

}

iface mecab {
    fn strerror() -> str;
    fn sparse_tostr(input: str) -> option::t<str>;
    fn get_dictionary_info() -> mecab_dictionary_info;
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

    fn get_dictionary_info() -> mecab_dictionary_info {
        let dict = _mecab::mecab_dictionary_info(self);
        {mutable base: dict} as mecab_dictionary_info
    }

}

impl <T: mecab, C> of mecab for {base: T, cleanup: C} {

    fn strerror() -> str unsafe {
        self.base.strerror()
    }

    fn sparse_tostr(input: str) -> option::t<str> {
        self.base.sparse_tostr(input)
    }

    fn get_dictionary_info() -> mecab_dictionary_info {
        self.base.get_dictionary_info()
    }

}

/*

FIXME:

 write documentation.

*/

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

