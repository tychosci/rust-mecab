/*

mecab.rs - The implementation of MeCab bindings for Rust.

Copyright (C) 2012 Tycho Sci.

FIXME:

 write documentation.

*/

use std;

// -*- rust -*-
import option::{some, none};

export mecab_new, mecab_new2, mecab_do, mecab_version;
export mecab;
export mecab_dictionary_info;

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
    type mecab_path_t;
    type mecab_dictionary_info_t;

    // FIXME: add more functions.
    fn mecab_new(argc: ctypes::c_int, argv: *str::sbuf)
        -> *mecab_t;

    fn mecab_new2(arg: str::sbuf)
        -> *mecab_t;

    fn mecab_destroy(mecab: *mecab_t);

    fn mecab_strerror(mecab: *mecab_t)
        -> str::sbuf;

    fn mecab_do(argc: ctypes::c_int, argv: *str::sbuf)
        -> ctypes::c_int;

    fn mecab_sparse_tostr(mecab: *mecab_t, input: str::sbuf)
        -> str::sbuf;

    fn mecab_sparse_tostr2(mecab: *mecab_t,
                           input: str::sbuf,
                           len:   ctypes::size_t)
        -> str::sbuf;

    fn mecab_dictionary_info(mecab: *mecab_t)
        -> *::mecab_dictionary_info_t;

    fn mecab_version() -> str::sbuf;

}

/*

Type: mecab_node_t

same structure of `_mecab::mecab_node_t` that documented in
http://mecab.sourceforge.net/doxygen/structmecab__node__t.html

*/
type mecab_node_t =
    { prev:     *_mecab::mecab_node_t
    , next:     *_mecab::mecab_node_t
    , enext:    *_mecab::mecab_node_t
    , bnext:    *_mecab::mecab_node_t
    , rpath:    *_mecab::mecab_path_t
    , lpath:    *_mecab::mecab_path_t
    , surface:   str::sbuf
    , feature:   str::sbuf
    , id:        ctypes::c_uint
    , length:    u16
    , rlength:   u16
    , rcAttr:    u16
    , lcAttr:    u16
    , posid:     u16
    , char_type: u8
    , stat:      u8
    , isbest:    u8
    , alpha:     ctypes::c_float
    , beta:      ctypes::c_float
    , prob:      ctypes::c_float
    , wcost:     i16
    , cost:      ctypes::long
    };

/*

Type: mecab_dictionary_info_t

same structure of `_mecab::mecab_dictionary_info_t` that documented in
http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html

*/
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
    fn sparse_tostr(input: str)  -> option::t<str>;
    fn sparse_tostr2(input: str) -> option::t<str>;
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

    fn sparse_tostr2(input: str) -> option::t<str> unsafe {
        let len = str::byte_len(input) as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tostr2(self, buf, len)
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

    fn sparse_tostr2(input: str) -> option::t<str> unsafe {
        self.base.sparse_tostr2(input)
    }

    fn get_dictionary_info() -> mecab_dictionary_info {
        self.base.get_dictionary_info()
    }

}

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

fn mecab_new2(arg: str) -> mecab unsafe {
    let m = str::as_buf(arg) { |buf|
        _mecab::mecab_new2(buf)
    };
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

fn mecab_version() -> str unsafe {
    let vers = _mecab::mecab_version();
    str::from_cstr(vers)
}
