/*
mecab.rs - The implementation of MeCab bindings for Rust.

Copyright (C) 2012 Tycho Sci.

This binding is licensed under the same license of MeCab.
*/

use std;

export mecab_new, mecab_new2, mecab_do, mecab_version;
export mecab;
export mecab_node;
export mecab_dictionary_info;
export MECAB_NOR_NODE, MECAB_UNK_NODE;
export MECAB_BOS_NODE, MECAB_EOS_NODE, MECAB_EON_NODE;

enum mecab_t = ();
enum mecab_path_t = ();

/// same structure of `_mecab::mecab_node_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
enum mecab_node_t = {
    prev:      *mecab_node_t,
    next:      *mecab_node_t,
    enext:     *mecab_node_t,
    bnext:     *mecab_node_t,
    rpath:     *mecab_path_t,
    lpath:     *mecab_path_t,
    surface:   *libc::c_char,
    feature:   *libc::c_char,
    id:         libc::c_uint,
    length:     u16,
    rlength:    u16,
    rcAttr:     u16,
    lcAttr:     u16,
    posid:      u16,
    char_type:  u8,
    stat:       u8,
    isbest:     u8,
    alpha:      libc::c_float,
    beta:       libc::c_float,
    prob:       libc::c_float,
    wcost:      i16,
    cost:       libc::c_long,
};

/// same structure of `_mecab::mecab_dictionary_info_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
enum mecab_dictionary_info_t = {
    filename: *libc::c_char,
    charset:  *libc::c_char,
    size:      libc::c_uint,
    type:      libc::c_int,
    lsize:     libc::c_uint,
    rsize:     libc::c_uint,
    version:   u16,
    next:     *mecab_dictionary_info_t,
};

/// Parameters for `mecab_node_t.stat` Normal node
/// defined in the dictionary.
const MECAB_NOR_NODE: u8 = 0u8;

/// Parameters for `mecab_node_t.stat` Unknown node
/// not defined in the dictionary.
const MECAB_UNK_NODE: u8 = 1u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a beginning of the sentence.
const MECAB_BOS_NODE: u8 = 2u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a end of the sentence.
const MECAB_EOS_NODE: u8 = 3u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a end of the N-best enumeration.
const MECAB_EON_NODE: u8 = 4u8;

trait mecab_dictionary_info {
    fn bump();
    fn is_end() -> bool;
    fn iter(blk: fn(mecab_dictionary_info));
    fn get_filename() ->  str;
    fn get_charset()  ->  str;
    fn get_size()     -> uint;
    fn get_type()     ->  int;
    fn get_lsize()    -> uint;
    fn get_rsize()    -> uint;
    fn get_version()  -> uint;
}

impl *mecab_dictionary_info_t : mecab_dictionary_info {
    fn bump() { }
    fn is_end() -> bool { unsafe { self == ptr::null() } }
    fn iter(_blk: fn(mecab_dictionary_info)) { }
    fn get_filename() -> str {
        unsafe {
            let buf = (*self).filename;
            str::unsafe::from_c_str(buf)
        }
    }
    fn get_charset() -> str {
        unsafe {
            let buf = (*self).charset;
            str::unsafe::from_c_str(buf)
        }
    }
    fn get_size()    -> uint { unsafe { (*self).size    as uint } }
    fn get_type()    ->  int { unsafe { (*self).type    as  int } }
    fn get_lsize()   -> uint { unsafe { (*self).lsize   as uint } }
    fn get_rsize()   -> uint { unsafe { (*self).rsize   as uint } }
    fn get_version() -> uint { unsafe { (*self).version as uint } }
}

impl {mut base: *mecab_dictionary_info_t} : mecab_dictionary_info {
    fn bump() {
        unsafe {
            self.base = (*self.base).next as *mecab_dictionary_info_t;
        }
    }
    fn is_end() -> bool { unsafe { self.base.is_end() } }
    fn iter(blk: fn(mecab_dictionary_info)) {
        while !self.is_end() {
            // FIXME figure out how to avoid unnecessary copying
            blk(self as mecab_dictionary_info);
            self.bump();
        }
    }
    fn get_filename() ->  str { unsafe { self.base.get_filename() } }
    fn get_charset()  ->  str { unsafe { self.base.get_charset() } }
    fn get_size()     -> uint { unsafe { self.base.get_size() } }
    fn get_type()     ->  int { unsafe { self.base.get_type() } }
    fn get_lsize()    -> uint { unsafe { self.base.get_lsize() } }
    fn get_rsize()    -> uint { unsafe { self.base.get_rsize() } }
    fn get_version()  -> uint { unsafe { self.base.get_version() } }
}

trait mecab_node {
    fn bump();
    fn is_end() -> bool;
    fn iter(blk: fn(mecab_node));
    fn get_surface() -> str;
    fn get_feature() -> str;
    fn get_status()  ->  u8;
}

impl *mecab_node_t : mecab_node {
    fn bump() { }
    fn is_end() -> bool { unsafe { self == ptr::null() } }
    fn iter(_blk: fn(mecab_node)) { }
    fn get_surface() -> str {
        unsafe {
            let buf = (*self).surface;
            let begin = 0u;
            let end = (*self).length as uint;
            let s = str::unsafe::from_c_str(buf);

            str::slice(s, begin, end)
        }
    }
    fn get_feature() -> str {
        unsafe {
            let buf = (*self).feature;
            str::unsafe::from_c_str(buf)
        }
    }
    fn get_status() -> u8 { unsafe { (*self).stat } }
}

impl {mut base: *mecab_node_t} : mecab_node {
    fn bump() {
        unsafe {
            self.base = (*self.base).next as *mecab_node_t;
        }
    }
    fn is_end() -> bool { unsafe { self.base.is_end() } }
    fn iter(blk: fn(mecab_node)) {
        while !self.is_end() {
            // FIXME figure out how to avoid unnecessary copying
            blk(self as mecab_node);
            self.bump();
        }
    }
    fn get_surface() -> str { unsafe { self.base.get_surface() } }
    fn get_feature() -> str { unsafe { self.base.get_feature() } }
    fn get_status()  ->  u8 { unsafe { self.base.get_status() } }
}

trait mecab {
    fn strerror() -> str;
    fn sparse_tostr(input: str) -> option<str>;
    fn sparse_tostr2(input: str, len: uint) -> option<str>;
    fn nbest_sparse_tostr(n: uint, input: str) -> option<str>;
    fn nbest_sparse_tostr2(n: uint, input: str, len: uint) -> option<str>;
    fn sparse_tonode(input: str) -> option<mecab_node>;
    fn sparse_tonode2(input: str, len: uint) -> option<mecab_node>;
    fn nbest_init(input: str) -> bool;
    fn nbest_init2(input: str, len: uint) -> bool;
    fn nbest_next_tostr() -> option<str>;
    fn nbest_next_tostr2(len: uint) -> option<str>;
    fn nbest_upto(to: uint, blk: fn(mecab));
    fn get_dictionary_info() -> option<mecab_dictionary_info>;
}

impl *mecab_t : mecab {
    fn strerror() -> str {
        unsafe {
            let res = _mecab::mecab_strerror(self);
            str::unsafe::from_c_str(res)
        }
    }
    fn sparse_tostr(input: str) -> option<str> {
        unsafe {
            let res = do str::as_buf(input)  |buf| {
                _mecab::mecab_sparse_tostr(self, buf)
            };

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn sparse_tostr2(input: str, len: uint) -> option<str> {
        unsafe {
            let len = len as libc::size_t;
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_sparse_tostr2(self, buf, len)
            };

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn nbest_sparse_tostr(n: uint, input: str) -> option<str> {
        unsafe {
            let n = n as libc::size_t;
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_nbest_sparse_tostr(self, n, buf)
            };

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn nbest_sparse_tostr2(n: uint, input: str, len: uint) -> option<str> {
        unsafe {
            let n = n as libc::size_t;
            let len = len as libc::size_t;
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_nbest_sparse_tostr2(self, n, buf, len)
            };

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn sparse_tonode(input: str) -> option<mecab_node> {
        unsafe {
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_sparse_tonode(self, buf)
            };

            if res == ptr::null() {
                none::<mecab_node>
            } else {
                some::<mecab_node>({mut base: res} as mecab_node)
            }
        }
    }
    fn sparse_tonode2(input: str, len: uint) -> option<mecab_node> {
        unsafe {
            let len = len as libc::size_t;
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_sparse_tonode2(self, buf, len)
            };

            if res == ptr::null() {
                none::<mecab_node>
            } else {
                some::<mecab_node>({mut base: res} as mecab_node)
            }
        }
    }
    fn nbest_init(input: str) -> bool {
        unsafe {
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_nbest_init(self, buf)
            };

            res as int != 0
        }
    }
    fn nbest_init2(input: str, len: uint) -> bool {
        unsafe {
            let len = len as libc::size_t;
            let res = do str::as_buf(input) |buf| {
                _mecab::mecab_nbest_init2(self, buf, len)
            };

            res as int != 0
        }
    }
    fn nbest_next_tostr() -> option<str> {
        unsafe {
            let res = _mecab::mecab_nbest_next_tostr(self);

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn nbest_next_tostr2(len: uint) -> option<str> {
        unsafe {
            let len = len as libc::size_t;
            let res = _mecab::mecab_nbest_next_tostr2(self, len);

            if res == ptr::null() {
                none::<str>
            } else {
                some::<str>(str::unsafe::from_c_str(res))
            }
        }
    }
    fn nbest_upto(to: uint, blk: fn(mecab)) {
        let mut to = to;
        while to > 0u {
            blk(self as mecab);
            to -= 1u;
        }
    }
    fn get_dictionary_info() -> option<mecab_dictionary_info> {
        let dict = _mecab::mecab_dictionary_info(self);
        if dict == ptr::null() {
            none
        } else {
            some({mut base: dict} as mecab_dictionary_info)
        }
    }
}

impl<T: mecab, C> {base: T, cleanup: C} : mecab {
    fn strerror() -> str {
        self.base.strerror()
    }
    fn sparse_tostr(input: str) -> option<str> {
        self.base.sparse_tostr(input)
    }
    fn sparse_tostr2(input: str, len: uint) -> option<str> {
        self.base.sparse_tostr2(input, len)
    }
    fn nbest_sparse_tostr(n: uint, input: str) -> option<str> {
        self.base.nbest_sparse_tostr(n, input)
    }
    fn nbest_sparse_tostr2(n: uint, input: str, len: uint) -> option<str> {
        self.base.nbest_sparse_tostr2(n, input, len)
    }
    fn sparse_tonode(input: str) -> option<mecab_node> {
        self.base.sparse_tonode(input)
    }
    fn sparse_tonode2(input: str, len: uint) -> option<mecab_node> {
        self.base.sparse_tonode2(input, len)
    }
    fn nbest_init(input: str) -> bool {
        self.base.nbest_init(input)
    }
    fn nbest_init2(input: str, len: uint) -> bool {
        self.base.nbest_init2(input, len)
    }
    fn nbest_next_tostr() -> option<str> {
        self.base.nbest_next_tostr()
    }
    fn nbest_next_tostr2(len: uint) -> option<str> {
        self.base.nbest_next_tostr2(len)
    }
    fn nbest_upto(to: uint, blk: fn(mecab)) {
        self.base.nbest_upto(to, blk);
    }
    fn get_dictionary_info() -> option<mecab_dictionary_info> {
        self.base.get_dictionary_info()
    }
}

class wrapped_mecab {
    let m: *mecab_t;
    new(m: *mecab_t) { self.m = m; }
    drop { _mecab::mecab_destroy(self.m); }
}

/// The wrapper of `_mecab::mecab_new` that returns wrapped structure `mecab`.
fn mecab_new(args: &[str]) -> option<mecab> {
    unsafe {
        let mut argptrs = ~[];
        let mut tmps = ~[];
        for vec::each(args) |arg| {
            let t = @arg;
            vec::push(tmps, t);
            vec::push_all(argptrs, str::as_c_str(*t, |b| ~[b]));
        }
        vec::push(argptrs, ptr::null());

        let m = vec::as_buf(argptrs, |argv, argc| {
            _mecab::mecab_new(argc, argv)
        });

        if m == ptr::null() {
            none::<mecab>
        } else {
            some::<mecab>({base: m, cleanup: wrapped_mecab(m)} as mecab)
        }
    }
}

/// The wrapper of `_mecab::mecab_new2` that returns wrapped structure `mecab`.
fn mecab_new2(arg: str) -> option<mecab> {
    unsafe {
        let m = str::as_c_str(arg, |buf| _mecab::mecab_new2(buf));

        if m == ptr::null() {
            none::<mecab>
        } else {
            some::<mecab>({base: m, cleanup: wrapped_mecab(m)} as mecab)
        }
    }
}

fn mecab_do(args: &[str]) -> int {
    unsafe {
        let mut argptrs = ~[];
        let mut tmps = ~[];
        for vec::each(args) |arg| {
            let t = @arg;
            vec::push(tmps, t);
            vec::push_all(argptrs, str::as_c_str(*t, |b| ~[b]));
        }
        vec::push(argptrs, ptr::null());

        let res = vec::as_buf(argptrs, |argv, argc| {
            _mecab::mecab_do(argc, argv)
        });

        res as int
    }
}

/// the wrapper of `_mecab::mecab_version` that returns version-number string.
fn mecab_version() -> str {
    unsafe {
        let vers = _mecab::mecab_version();
        str::unsafe::from_c_str(vers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mecab_version() {
        assert 0u != str::char_len(mecab_version());
    }
    #[test]
    fn test_sparse_tostr() {
        let m = match mecab_new2("") {
            some(_m) => _m,
            none     => fail
        };
        let s = "いつもより大きなリンゴを仕入れることが出来た";
        let r = m.sparse_tostr(s);

        match r {
            some(i) => { assert 0u != str::char_len(i); }
            none    => { assert false; }
        }
    }
    #[test]
    fn test_sparse_tostr2() {
        let m = match mecab_new2("") {
            some(_m) => _m,
            none     => fail
        };
        let s = "これはパースするための文です";
        let r = m.sparse_tostr2(s, str::len(s));

        match r {
            some::<str>(i) => { assert 0u != str::char_len(i); }
            none::<str>    => { assert false; }
        }
    }
    #[test]
    fn test_mecab_node_iter() {
        let m = match mecab_new2("") {
            some::<mecab>(_m) => _m,
            none::<mecab>     => fail
        };
        let s = "もももすももももものうち";
        let r = m.sparse_tonode(s);

        match r {
            some(node) => {
                do node.iter |_n| { }
                assert true;
            }
            none => {
                assert false;
            }
        }
    }
}

#[link_name = "mecab"]
#[abi = "cdecl"]
extern mod _mecab {
    fn mecab_new(argc: libc::c_int, argv: **u8) -> *::mecab_t;
    fn mecab_new2(arg: *libc::c_char) -> *::mecab_t;
    fn mecab_destroy(mecab: *::mecab_t);
    fn mecab_strerror(mecab: *::mecab_t) -> *libc::c_char;
    fn mecab_do(argc: libc::c_int, argv: **u8) -> libc::c_int;
    fn mecab_sparse_tostr(mecab: *::mecab_t, input: *u8) -> *libc::c_char;
    fn mecab_sparse_tostr2(mecab: *::mecab_t, input: *u8, len: libc::size_t) -> *libc::c_char;
    fn mecab_nbest_sparse_tostr(mecab: *::mecab_t, n: libc::size_t, input: *u8) -> *libc::c_char;
    fn mecab_nbest_sparse_tostr2(mecab: *::mecab_t, n: libc::size_t, input: *u8, len: libc::size_t) -> *libc::c_char;
    fn mecab_sparse_tonode(mecab: *::mecab_t, input: *u8) -> *::mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *::mecab_t, input: *u8, len: libc::size_t) -> *::mecab_node_t;
    fn mecab_nbest_init(mecab: *::mecab_t, input: *u8) -> libc::c_int;
    fn mecab_nbest_init2(mecab: *::mecab_t, input: *u8, len:   libc::size_t) -> libc::c_int;
    fn mecab_nbest_next_tostr(mecab: *::mecab_t) -> *libc::c_char;
    fn mecab_nbest_next_tostr2(mecab: *::mecab_t, len: libc::size_t) -> *libc::c_char;
    fn mecab_dictionary_info(mecab: *::mecab_t) -> *::mecab_dictionary_info_t;
    fn mecab_version() -> *libc::c_char;
}
