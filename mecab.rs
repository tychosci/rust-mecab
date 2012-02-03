/*

mecab.rs - The implementation of MeCab bindings for Rust.

Copyright (C) 2012 Tycho Sci.

This binding is licensed under the same license of MeCab.

FIXME:

 write documentation.

*/

use std;

export mecab_new, mecab_new2, mecab_do, mecab_version;
export mecab;
export mecab_node;
export mecab_dictionary_info;
export MECAB_NOR_NODE, MECAB_UNK_NODE;
export MECAB_BOS_NODE, MECAB_EOS_NODE, MECAB_EON_NODE;

// dummy enums for recursive structures and opaque pointers
enum _mecab_t { }
enum _mecab_node_t { }
enum _mecab_path_t { }
enum _mecab_dictionary_info_t { }

#[link_name = "mecab"]
#[abi = "cdecl"]
native mod _mecab {

    // FIXME: add more functions.
    fn mecab_new(argc: ctypes::c_int, argv: **u8)
        -> *::_mecab_t;

    fn mecab_new2(arg: *u8)
        -> *::_mecab_t;

    fn mecab_destroy(mecab: *::_mecab_t);

    fn mecab_strerror(mecab: *::_mecab_t)
        -> *u8;

    fn mecab_do(argc: ctypes::c_int, argv: **u8)
        -> ctypes::c_int;

    fn mecab_sparse_tostr(mecab: *::_mecab_t, input: *u8)
        -> *u8;

    fn mecab_sparse_tostr2(mecab: *::_mecab_t,
                           input: *u8,
                           len:   ctypes::size_t)
        -> *u8;

    fn mecab_nbest_sparse_tostr(mecab: *::_mecab_t,
                                n:     ctypes::size_t,
                                input: *u8)
        -> *u8;

    fn mecab_nbest_sparse_tostr2(mecab: *::_mecab_t,
                                 n:     ctypes::size_t,
                                 input: *u8,
                                 len:   ctypes::size_t)
        -> *u8;

    fn mecab_sparse_tonode(mecab: *::_mecab_t, input: *u8)
        -> *::mecab_node_t;

    fn mecab_sparse_tonode2(mecab: *::_mecab_t,
                            input: *u8,
                            len:   ctypes::size_t)
        -> *::mecab_node_t;

    fn mecab_nbest_init(mecab: *::_mecab_t, input: *u8)
        -> ctypes::c_int;

    fn mecab_nbest_init2(mecab: *::_mecab_t,
                         input: *u8,
                         len:   ctypes::size_t)
        -> ctypes::c_int;

    fn mecab_nbest_next_tostr(mecab: *::_mecab_t)
        -> *u8;

    fn mecab_nbest_next_tostr2(mecab: *::_mecab_t, len: ctypes::size_t)
        -> *u8;

    fn mecab_dictionary_info(mecab: *::_mecab_t)
        -> *::mecab_dictionary_info_t;

    fn mecab_version() -> *u8;

}

/*

Type: mecab_node_t

same structure of `_mecab::mecab_node_t` that documented in
http://mecab.sourceforge.net/doxygen/structmecab__node__t.html

*/
type mecab_node_t =
    { prev:      *_mecab_node_t
    , next:      *_mecab_node_t
    , enext:     *_mecab_node_t
    , bnext:     *_mecab_node_t
    , rpath:     *_mecab_path_t
    , lpath:     *_mecab_path_t
    , surface:   *u8
    , feature:   *u8
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
    { filename: *u8
    , charset:  *u8
    , size:     ctypes::c_uint
    , type:     ctypes::c_int
    , lsize:    ctypes::c_uint
    , rsize:    ctypes::c_uint
    , version:  u16
    , next:     *_mecab_dictionary_info_t
    };

/*

Constant: MECAB_NOR_NODE

Parameters for `mecab_node_t.stat`
Normal node defined in the dictionary.

*/
const MECAB_NOR_NODE: u8 = 0u8;

/*

Constant: MECAB_UNK_NODE

Parameters for `mecab_node_t.stat`
Unknown node not defined in the dictionary.

*/
const MECAB_UNK_NODE: u8 = 1u8;

/*

Constant: MECAB_BOS_NODE

Parameters for `mecab_node_t.stat`
Virtual node representing a beginning of the sentence.

*/
const MECAB_BOS_NODE: u8 = 2u8;

/*

Constant: MECAB_EOS_NODE

Parameters for `mecab_node_t.stat`
Virtual node representing a end of the sentence.

*/
const MECAB_EOS_NODE: u8 = 3u8;

/*

Constant: MECAB_EON_NODE

Parameters for `mecab_node_t.stat`
Virtual node representing a end of the N-best enumeration.

*/
const MECAB_EON_NODE: u8 = 4u8;

/*

Interface: mecab_dictionary_info

FIXME: write

*/
iface mecab_dictionary_info {
    fn bump();

    fn is_end() -> bool;

    fn iter(blk: fn(mecab_dictionary_info));

    fn get_filename() -> str;
    fn get_charset()  -> str;

    fn get_size()    -> uint;
    fn get_type()    ->  int;
    fn get_lsize()   -> uint;
    fn get_rsize()   -> uint;
    fn get_version() -> uint;
}

impl of mecab_dictionary_info for *mecab_dictionary_info_t {

    fn bump() unsafe { }

    fn is_end() -> bool unsafe { self == ptr::null() }

    fn iter(_blk: fn(mecab_dictionary_info)) { }

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

    fn iter(blk: fn(mecab_dictionary_info)) {
        while !self.is_end() {
            blk(self as mecab_dictionary_info);
            self.bump();
        }
    }

    fn get_filename() -> str unsafe { self.base.get_filename() }

    fn get_charset()  -> str unsafe { self.base.get_charset() }

    fn get_size()     -> uint unsafe { self.base.get_size() }
    fn get_type()     ->  int unsafe { self.base.get_type() }
    fn get_lsize()    -> uint unsafe { self.base.get_lsize() }
    fn get_rsize()    -> uint unsafe { self.base.get_rsize() }
    fn get_version()  -> uint unsafe { self.base.get_version() }

}

/*

Interface: mecab_node

FIXME: write

*/
iface mecab_node {
    fn bump();

    fn is_end() -> bool;

    fn iter(blk: fn(mecab_node));

    fn get_surface() -> str;
    fn get_feature() -> str;

    fn get_status() -> u8;
}

impl of mecab_node for *mecab_node_t {

    fn bump() unsafe { }

    fn is_end() -> bool unsafe { self == ptr::null() }

    fn iter(_blk: fn(mecab_node)) { }

    fn get_surface() -> str unsafe {
        let buf = (*self).surface;
        let begin = 0u;
        let end = (*self).length as uint;
        let s = str::from_cstr(buf);

        check uint::le(begin, end);

        str::substr(s, begin, end)
    }

    fn get_feature() -> str unsafe {
        let buf = (*self).feature;
        str::from_cstr(buf)
    }

    fn get_status() -> u8 unsafe { (*self).stat }

}

impl of mecab_node for {mutable base: *mecab_node_t} {

    fn bump() unsafe {
        self.base = (*self.base).next as *mecab_node_t;
    }

    fn is_end() -> bool { self.base.is_end() }


    fn iter(blk: fn(mecab_node)) {
        while !self.is_end() {
            blk(self as mecab_node);
            self.bump();
        }
    }

    fn get_surface() -> str unsafe {
        self.base.get_surface()
    }

    fn get_feature() -> str unsafe {
        self.base.get_feature()
    }

    fn get_status() -> u8 unsafe {
        self.base.get_status()
    }

}

/*

Interface: mecab

FIXME: write

*/
iface mecab {
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

impl of mecab for *_mecab_t {

    fn strerror() -> str unsafe {
        let res = _mecab::mecab_strerror(self);
        str::from_cstr(res)
    }

    fn sparse_tostr(input: str) -> option<str> unsafe {
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tostr(self, buf)
        };

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn sparse_tostr2(input: str, len: uint) -> option<str> unsafe {
        let len = len as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tostr2(self, buf, len)
        };

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn nbest_sparse_tostr(n: uint, input: str) -> option<str> unsafe {
        let n = n as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_nbest_sparse_tostr(self, n, buf)
        };

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn nbest_sparse_tostr2(n: uint, input: str, len: uint)
        -> option<str> unsafe {
        let n = n as ctypes::size_t;
        let len = len as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_nbest_sparse_tostr2(self, n, buf, len)
        };

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn sparse_tonode(input: str) -> option<mecab_node> unsafe {
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tonode(self, buf)
        };

        if res == ptr::null() {
            none::<mecab_node>
        } else {
            some::<mecab_node>({mutable base: res} as mecab_node)
        }
    }

    fn sparse_tonode2(input: str, len: uint) -> option<mecab_node> unsafe {
        let len = len as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_sparse_tonode2(self, buf, len)
        };

        if res == ptr::null() {
            none::<mecab_node>
        } else {
            some::<mecab_node>({mutable base: res} as mecab_node)
        }
    }

    fn nbest_init(input: str) -> bool unsafe {
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_nbest_init(self, buf)
        };

        res as int != 0
    }

    fn nbest_init2(input: str, len: uint) -> bool unsafe {
        let len = len as ctypes::size_t;
        let res = str::as_buf(input) { |buf|
            _mecab::mecab_nbest_init2(self, buf, len)
        };

        res as int != 0
    }

    fn nbest_next_tostr() -> option<str> unsafe {
        let res = _mecab::mecab_nbest_next_tostr(self);

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn nbest_next_tostr2(len: uint) -> option<str> unsafe {
        let len = len as ctypes::size_t;
        let res = _mecab::mecab_nbest_next_tostr2(self, len);

        if res == ptr::null() {
            none::<str>
        } else {
            some::<str>(str::from_cstr(res))
        }
    }

    fn nbest_upto(to: uint, blk: fn(mecab)) {
        let to = to;
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
            some({mutable base: dict} as mecab_dictionary_info)
        }
    }

}

impl <T: mecab, C> of mecab for {base: T, cleanup: C} {

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

resource wrapped_mecab(m: *_mecab_t) {
    _mecab::mecab_destroy(m);
}

/*

Function: mecab_new

the wrapper of `_mecab::mecab_new` that returns wrapped structure `mecab`

*/
fn mecab_new(args: [str]) -> option<mecab> unsafe {
    let argc = vec::len(args) as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let m = _mecab::mecab_new(argc, vec::unsafe::to_ptr(argv));

    if m == ptr::null() {
        none::<mecab>
    } else {
        some::<mecab>({base: m, cleanup: wrapped_mecab(m)} as mecab)
    }
}

/*

Function: mecab_new

the wrapper of `_mecab::mecab_new2` that returns wrapped structure `mecab`

*/
fn mecab_new2(arg: str) -> option<mecab> unsafe {
    let m = str::as_buf(arg) { |buf|
        _mecab::mecab_new2(buf)
    };

    if m == ptr::null() {
        none::<mecab>
    } else {
        some::<mecab>({base: m, cleanup: wrapped_mecab(m)} as mecab)
    }
}

/*

Function: mecab_do

the wrapper of `_mecab::mecab_do` that returns status code at termination

*/
fn mecab_do(args: [str]) -> int unsafe {
    let argc = vec::len(args) as ctypes::c_int;

    let argv = [];
    for arg in args {
        argv += str::as_buf(arg) { |buf| [buf] };
    }
    argv += [ptr::null()];

    let res = _mecab::mecab_do(argc, vec::unsafe::to_ptr(argv));
    res as int
}

/*

Function: mecab_version

the wrapper of `_mecab::mecab_version` that returns version-number string

*/
fn mecab_version() -> str unsafe {
    let vers = _mecab::mecab_version();
    str::from_cstr(vers)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_mecab_version() {
        assert 0u != str::char_len(mecab_version());
    }

    #[test]
    fn test_sparse_tostr() {
        let m = alt mecab_new2("") {
          some::<mecab>(_m) { _m }
          none::<mecab>     { fail; }
        };
        let s = "いつもより大きなリンゴを仕入れることが出来た";
        let r = m.sparse_tostr(s);

        alt r {
          some::<str>(i) { assert 0u != str::char_len(i); }
          none::<str>    { assert false; }
        }
    }

    #[test]
    fn test_sparse_tostr2() {
        let m = alt mecab_new2("") {
          some::<mecab>(_m) { _m }
          none::<mecab>     { fail; }
        };
        let s = "これはパースするための文です";
        let r = m.sparse_tostr2(s, str::byte_len(s));

        alt r {
          some::<str>(i) { assert 0u != str::char_len(i); }
          none::<str>    { assert false; }
        }
    }

    #[test]
    fn test_mecab_node_iter() {
        let m = alt mecab_new2("") {
          some::<mecab>(_m) { _m }
          none::<mecab>     { fail; }
        };
        let s = "もももすももももものうち";
        let r = m.sparse_tonode(s);

        alt r {
          some::<mecab_node>(node) {
            node.iter { |_n| }
            assert true;
          }
          none::<mecab_node> {
            assert false;
          }
        }
    }

}
