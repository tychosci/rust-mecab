//
// mecab.rs - The implementation of MeCab bindings for Rust.
//
// Copyright (C) 2012 Tycho Sci.
//
// This binding is licensed under the same license of MeCab.
//

import str::unsafe;
import libc::*;

enum mecab_t = ();
enum mecab_path_t = ();

/// same structure of `mecab::mecab_node_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
enum mecab_node_t = {
    prev:      *mecab_node_t,
    next:      *mecab_node_t,
    enext:     *mecab_node_t,
    bnext:     *mecab_node_t,
    rpath:     *mecab_path_t,
    lpath:     *mecab_path_t,
    surface:   *c_char,
    feature:   *c_char,
    id:         c_uint,
    length:     u16,
    rlength:    u16,
    rcAttr:     u16,
    lcAttr:     u16,
    posid:      u16,
    char_type:  u8,
    stat:       u8,
    isbest:     u8,
    alpha:      c_float,
    beta:       c_float,
    prob:       c_float,
    wcost:      i16,
    cost:       c_long,
};

/// same structure of `mecab::mecab_dictionary_info_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
enum mecab_dictionary_info_t = {
    filename: *c_char,
    charset:  *c_char,
    size:      c_uint,
    type:      c_int,
    lsize:     c_uint,
    rsize:     c_uint,
    version:   u16,
    next:     *mecab_dictionary_info_t,
};

struct MecabDictionaryInfo {
    mut dict: *mecab_dictionary_info_t;
    head:     *mecab_dictionary_info_t;
}

struct MecabNode {
    mut node: *mecab_node_t;
    head:     *mecab_node_t;
}

trait IMecabDictionaryInfo {
    pure fn get_filename() -> ~str;
    pure fn get_charset()  -> ~str;
    pure fn get_size()     -> uint;
    pure fn get_type()     ->  int;
    pure fn get_lsize()    -> uint;
    pure fn get_rsize()    -> uint;
    pure fn get_version()  -> uint;
}

trait IMecabNode {
    pure fn get_surface() -> ~str;
    pure fn get_feature() -> ~str;
    pure fn get_status()  -> u8;
}

impl *mecab_dictionary_info_t : IMecabDictionaryInfo {
    pure fn get_filename() -> ~str { unsafe { unsafe::from_c_str((*self).filename) } }
    pure fn get_charset()  -> ~str { unsafe { unsafe::from_c_str((*self).charset)  } }
    pure fn get_size()     -> uint { unsafe { (*self).size    as uint } }
    pure fn get_type()     ->  int { unsafe { (*self).type    as  int } }
    pure fn get_lsize()    -> uint { unsafe { (*self).lsize   as uint } }
    pure fn get_rsize()    -> uint { unsafe { (*self).rsize   as uint } }
    pure fn get_version()  -> uint { unsafe { (*self).version as uint } }
}

impl *mecab_node_t : IMecabNode {
    pure fn get_surface() -> ~str {
        unsafe {
            let s = str::unsafe::from_c_str((*self).surface);
            str::slice(s, 0, (*self).length as uint)
        }
    }
    pure fn get_feature() -> ~str {
        unsafe { str::unsafe::from_c_str((*self).feature) }
    }
    pure fn get_status() -> u8 {
        unsafe { (*self).stat }
    }
}

impl MecabDictionaryInfo {
    fn each(blk: fn(*mecab_dictionary_info_t) -> bool) {
        let mut p = self.dict;
        unsafe {
            while p.is_not_null() {
                if !blk(p) { break; }
                p = (*p).next;
            }
        }
    }
    fn next() -> bool {
        unsafe { self.dict = (*self.dict).next; }
        self.end()
    }
    fn reset() {
        self.dict = self.head;
    }
    fn end() -> bool {
        unsafe { self.dict.is_null() }
    }
}

impl MecabNode {
    fn each(blk: fn(*mecab_node_t) -> bool) {
        let mut p = self.node;
        unsafe {
            while p.is_not_null() {
                if !blk(p) { break; }
                p = (*p).next;
            }
        }
    }
    fn next() -> bool {
        unsafe { self.node = (*self.node).next; }
        self.end()
    }
    fn reset() {
        self.node = self.head;
    }
    fn end() -> bool {
        unsafe { self.node.is_null() }
    }
}

class Mecab {
    let mecab: *mecab_t;

    new(mecab: *mecab_t) { self.mecab = mecab; }

    drop { mecab::mecab_destroy(self.mecab); }

    fn get_dictionary_info() -> option<@MecabDictionaryInfo> {
        let dict = mecab::mecab_dictionary_info(self.mecab);

        if dict.is_null() {
            none
        } else {
            some(@MecabDictionaryInfo {
                dict: dict,
                head: dict,
            })
        }
    }
}

/// The wrapper of `mecab::mecab_new` that may return `Mecab`.
fn mecab_new(args: &a/[&str]) -> option<@Mecab> {
    let argc = args.len() as c_int;

    unsafe {
        let mut argptrs = ~[];
        let mut tmps    = ~[];

        for args.each |arg| {
            let t = @arg;
            vec::push(tmps, t);
            vec::push_all(argptrs, str::as_c_str(*t, |b| ~[b]));
        }
        vec::push(argptrs, ptr::null());

        let m = vec::as_buf(argptrs, |argv, _len| {
            mecab::mecab_new(argc, argv)
        });

        if m.is_null() {
            none
        } else {
            some(@Mecab(m))
        }
    }
}

/// The wrapper of `mecab::mecab_new2` that may return `Mecab`.
fn mecab_new2(arg: &a/str) -> option<@Mecab> {
    unsafe {
        let m = str::as_c_str(arg, |buf| mecab::mecab_new2(buf));

        if m.is_null() {
            none
        } else {
            some(@Mecab(m))
        }
    }
}

/// the wrapper of `mecab::mecab_version` that returns version-number string.
fn mecab_version() -> ~str {
    unsafe {
        let vers = mecab::mecab_version();
        str::unsafe::from_c_str(vers)
    }
}

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

// NB: Need to expand `mecab-config --libs-only-L` at linking time
extern mod mecab {
    fn mecab_new(argc: c_int, argv: **c_char) -> *mecab_t;
    fn mecab_new2(arg: *c_char) -> *mecab_t;
    fn mecab_destroy(mecab: *mecab_t);
    fn mecab_strerror(mecab: *mecab_t) -> *c_char;
    fn mecab_do(argc: c_int, argv: **u8) -> c_int;
    fn mecab_sparse_tostr(mecab: *mecab_t, input: *u8) -> *c_char;
    fn mecab_sparse_tostr2(mecab: *mecab_t, input: *u8, len: size_t) -> *c_char;
    fn mecab_nbest_sparse_tostr(mecab: *mecab_t, n: size_t, input: *u8) -> *c_char;
    fn mecab_nbest_sparse_tostr2(mecab: *mecab_t, n: size_t, input: *u8, len: size_t) -> *c_char;
    fn mecab_sparse_tonode(mecab: *mecab_t, input: *u8) -> *mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *mecab_t, input: *u8, len: size_t) -> *mecab_node_t;
    fn mecab_nbest_init(mecab: *mecab_t, input: *u8) -> c_int;
    fn mecab_nbest_init2(mecab: *mecab_t, input: *u8, len: size_t) -> c_int;
    fn mecab_nbest_next_tostr(mecab: *mecab_t) -> *c_char;
    fn mecab_nbest_next_tostr2(mecab: *mecab_t, len: size_t) -> *c_char;
    fn mecab_dictionary_info(mecab: *mecab_t) -> *mecab_dictionary_info_t;
    fn mecab_version() -> *c_char;
}
