/*!
MeCab bindings for Rust

Copyright (C) 2012 Tycho Sci

This binding is licensed under the same license of MeCab.
*/

#[link(name = "mecab",
       vers = "0.2",
       uuid = "157601c8-818c-4898-b1dc-890eeeab4936",
       url  = "https://github.com/tychosci/rust-mecab")];

#[comment = "MeCab bindings for Rust"];
#[license = "GPL/LGPL/BSD"];
#[crate_type = "lib"];

extern mod std;

use core::str::raw;
use core::libc::*;

#[cfg(test)]
mod test;

// NB: Need to expand `mecab-config --libs-only-L` at link time
#[nolink]
#[link_args = "-lmecab -lstdc++"]
extern {
    fn mecab_new(argc: c_int, argv: **c_char) -> *mecab_t;
    fn mecab_new2(arg: *c_char) -> *mecab_t;
    fn mecab_destroy(mecab: *mecab_t);
    fn mecab_strerror(mecab: *mecab_t) -> *c_char;
    fn mecab_do(argc: c_int, argv: **c_char) -> c_int;
    fn mecab_sparse_tostr(mecab: *mecab_t, input: *c_char) -> *c_char;
    fn mecab_sparse_tostr2(mecab: *mecab_t, input: *c_char, len: size_t) -> *c_char;
    fn mecab_sparse_tonode(mecab: *mecab_t, input: *c_char) -> *mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *mecab_t, input: *c_char, len: size_t) -> *mecab_node_t;
    fn mecab_parse_lattice(mecab: *mecab_t, lattice: *mecab_lattice_t) -> c_int;
    fn mecab_dictionary_info(mecab: *mecab_t) -> *mecab_dictionary_info_t;
    fn mecab_version() -> *c_char;

    fn mecab_model_new(argc: c_int, argv: **c_char) -> *mecab_model_t;
    fn mecab_model_new2(arg: *c_char) -> *mecab_model_t;
    fn mecab_model_new_tagger(model: *mecab_model_t) -> *mecab_t;
    fn mecab_model_new_lattice(model: *mecab_model_t) -> *mecab_lattice_t;
    fn mecab_model_destroy(model: *mecab_model_t);

    fn mecab_lattice_set_sentence(lattice: *mecab_lattice_t, input: *c_char);
    fn mecab_lattice_tostr(lattice: *mecab_lattice_t) -> *c_char;
    fn mecab_lattice_get_size(lattice: *mecab_lattice_t) -> size_t;
    fn mecab_lattice_get_bos_node(lattice: *mecab_lattice_t) -> *mecab_node_t;
    fn mecab_lattice_get_eos_node(lattice: *mecab_lattice_t) -> *mecab_node_t;
    fn mecab_lattice_get_begin_nodes(lattice: *mecab_lattice_t, pos: size_t) -> *mecab_node_t;
    fn mecab_lattice_get_end_nodes(lattice: *mecab_lattice_t, pos: size_t) -> *mecab_node_t;
    fn mecab_lattice_destroy(lattice: *mecab_lattice_t);
    fn mecab_lattice_strerror(lattice: *mecab_lattice_t) -> *c_char;
}

#[allow(non_camel_case_types)]
struct mecab_t;

#[allow(non_camel_case_types)]
struct mecab_model_t;

#[allow(non_camel_case_types)]
struct mecab_lattice_t;

/**
Same structure of `mecab::mecab_path_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__path__t.html>
*/
#[allow(non_camel_case_types)]
struct mecab_path_t {
    rnode: *mecab_node_t,
    rnext: *mecab_path_t,
    lnode: *mecab_node_t,
    lnext: *mecab_path_t,
    cost:   c_int,
    prob:   c_float,
}

/**
Same structure of `mecab::mecab_node_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
*/
#[allow(non_camel_case_types)]
struct mecab_node_t {
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
}

/**
Same structure of `mecab::mecab_dictionary_info_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
*/
#[allow(non_camel_case_types)]
struct mecab_dictionary_info_t {
    filename: *c_char,
    charset:  *c_char,
    size:      c_uint,
    ty:        c_int,
    lsize:     c_uint,
    rsize:     c_uint,
    version:   u16,
    next:     *mecab_dictionary_info_t,
}

/**
Parameters for `mecab_node_t.stat` Normal node
defined in the dictionary.
*/
pub const NOR_NODE: u8 = 0u8;

/**
Parameters for `mecab_node_t.stat` Unknown node
not defined in the dictionary.
*/
pub const UNK_NODE: u8 = 1u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a beginning of the sentence.
*/
pub const BOS_NODE: u8 = 2u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the sentence.
*/
pub const EOS_NODE: u8 = 3u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the N-best enumeration.
*/
pub const EON_NODE: u8 = 4u8;

/// Wrapped structure for `mecab_dictionary_info_t`.
pub struct MeCabDictionaryInfo {
    priv dict: *mecab_dictionary_info_t
}

/// Wrapped structure for `mecab_node_t`.
pub struct MeCabNode {
    priv node: *mecab_node_t
}

/// Wrapped structure for `mecab_t`.
pub struct MeCab {
    priv mecab: *mecab_t
}

/// Wrapped structure for `mecab_model_t`.
pub struct MeCabModel {
    priv model: *mecab_model_t
}

/// Wrapped structure for `mecab_lattice_t`.
pub struct MeCabLattice {
    pub lattice: *mecab_lattice_t
}

impl Drop for MeCabDictionaryInfo {
    fn finalize(&self) {}
}

impl Drop for MeCabNode {
    fn finalize(&self) {}
}

impl Drop for MeCab {
    fn finalize(&self) {
        unsafe { mecab_destroy(self.mecab); }
    }
}

impl Drop for MeCabModel {
    fn finalize(&self) {
        unsafe { mecab_model_destroy(self.model); }
    }
}

impl Drop for MeCabLattice {
    fn finalize(&self) {
        unsafe { mecab_lattice_destroy(self.lattice); }
    }
}

pub trait IMeCabDict {
    pure fn get_filename(&self) -> ~str;
    pure fn get_charset(&self) -> ~str;
    pure fn get_size(&self) -> uint;
    pure fn get_type(&self) -> int;
    pure fn get_lsize(&self) -> uint;
    pure fn get_rsize(&self) -> uint;
    pure fn get_version(&self) -> uint;
}

pub trait IMeCabNode {
    pure fn get_surface(&self) -> ~str;
    pure fn get_feature(&self) -> ~str;
    pure fn get_status(&self) -> u8;
    pure fn get_posid(&self) -> u16;
    pure fn get_prob(&self) -> c_float;

    pure fn is_best(&self) -> bool;
}

impl IMeCabDict for *mecab_dictionary_info_t {
    /// Returns `mecab_dictionary_info_t.filename`.
    pure fn get_filename(&self) -> ~str {
        unsafe { raw::from_c_str((**self).filename) }
    }

    /// Returns `mecab_dictionary_info_t.charset`.
    pure fn get_charset(&self) -> ~str {
        unsafe { raw::from_c_str((**self).charset) }
    }

    /// Returns `mecab_dictionary_info_t.size`.
    pure fn get_size(&self) -> uint {
        unsafe { (**self).size as uint }
    }

    /// Returns `mecab_dictionary_info_t.type`.
    pure fn get_type(&self) -> int {
        unsafe { (**self).ty as int }
    }

    /// Returns `mecab_dictionary_info_t.lsize`.
    pure fn get_lsize(&self) -> uint {
        unsafe { (**self).lsize as uint }
    }

    /// Returns `mecab_dictionary_info_t.rsize`.
    pure fn get_rsize(&self) -> uint {
        unsafe { (**self).rsize as uint }
    }

    /// Returns `mecab_dictionary_info_t.version`.
    pure fn get_version(&self) -> uint {
        unsafe { (**self).version as uint }
    }
}

impl IMeCabNode for *mecab_node_t {
    /// Returns pre-sliced `mecab_node_t.surface`.
    pure fn get_surface(&self) -> ~str {
        unsafe {
            let s = raw::from_c_str((**self).surface);
            str::slice(s, 0, (**self).length as uint)
        }
    }

    /// Returns `mecab_node_t.feature`.
    pure fn get_feature(&self) -> ~str {
        unsafe { raw::from_c_str((**self).feature) }
    }

    /// Returns `mecab_node_t.status`.
    pure fn get_status(&self) -> u8 {
        unsafe { (**self).stat }
    }

    /// Returns `mecab_node_t.posid`.
    pure fn get_posid(&self) -> u16 {
        unsafe { (**self).posid }
    }

    /// Returns `mecab_node_t.prob`.
    pure fn get_prob(&self) -> c_float {
        unsafe { (**self).prob }
    }

    pure fn is_best(&self) -> bool {
        unsafe { (**self).isbest == 1 }
    }
}

impl BaseIter<@IMeCabDict> for MeCabDictionaryInfo {
    pure fn size_hint(&self) -> Option<uint> { None }

    pure fn each(&self, blk: fn(&@IMeCabDict) -> bool) {
        let mut p = self.dict;

        while p.is_not_null() {
            let p_ = p as IMeCabDict;
            if !blk(&p_) { break; }
            unsafe { p = (*p).next; }
        }
    }
}

impl BaseIter<@IMeCabNode> for MeCabNode {
    pure fn size_hint(&self) -> Option<uint> { None }

    pure fn each(&self, blk: fn(&@IMeCabNode) -> bool) {
        let mut p = self.node;

        while p.is_not_null() {
            let p_ = p as IMeCabNode;
            if !blk(&p_) { break; }
            unsafe { p = (*p).next; }
        }
    }
}

impl MeCab {
    /// Parses input and may return the string of result.
    fn parse(&self, input: &str) -> ~str {
        let s = str::as_c_str(input, |buf| unsafe {
            mecab_sparse_tostr(self.mecab, buf)
        });

        if s.is_null() {
            let msg = self.strerror();
            fail!(msg);
        } else {
            unsafe { raw::from_c_str(s) }
        }
    }

    /// Parses input and may return `MeCabNode`.
    fn parse_to_node(&self, input: &str) -> MeCabNode {
        let node = str::as_c_str(input, |buf| unsafe {
            mecab_sparse_tonode(self.mecab, buf)
        });

        if node.is_null() {
            let msg = self.strerror();
            fail!(msg);
        } else {
            MeCabNode { node: node }
        }
    }

    /// Parses input in given `lattice` and returns true on success.
    fn parse_lattice(&self, lattice: &MeCabLattice) -> bool {
        unsafe {
            let status = mecab_parse_lattice(self.mecab, lattice.lattice);
            status != 0 as c_int
        }
    }

    /// Returns `MeCabDictionaryInfo`.
    fn get_dictionary_info(&self) -> MeCabDictionaryInfo {
        unsafe {
            let dict = mecab_dictionary_info(self.mecab);

            if dict.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                MeCabDictionaryInfo { dict: dict }
            }
        }
    }

    priv fn strerror(&self) -> ~str {
        unsafe {
            let s = mecab_strerror(self.mecab);
            raw::from_c_str(s)
        }
    }
}

impl MeCabModel {
    /// Creates new tagger.
    fn create_tagger(&self) -> MeCab {
        unsafe {
            let mecab = mecab_model_new_tagger(self.model);

            if mecab.is_null() {
                fail!(~"failed to create new Tagger");
            } else {
                MeCab { mecab: mecab }
            }
        }
    }

    /// Creates new lattice.
    fn create_lattice(&self) -> MeCabLattice {
        unsafe {
            let lattice = mecab_model_new_lattice(self.model);

            if lattice.is_null() {
                fail!(~"failed to create new Lattice");
            } else {
                MeCabLattice { lattice: lattice }
            }
        }
    }
}

impl ToStr for MeCabLattice {
    pure fn to_str(&self) -> ~str {
        unsafe {
            let s = mecab_lattice_tostr(self.lattice);
            raw::from_c_str(s)
        }
    }
}

impl MeCabLattice {
    /// Set input of the lattice.
    fn set_sentence(&self, input: &str) {
        do str::as_c_str(input) |buf| {
            unsafe { mecab_lattice_set_sentence(self.lattice, buf); }
        }
    }

    /// Returns the beginning node of the sentence on success.
    fn get_bos_node(&self) -> MeCabNode {
        unsafe {
            let node = mecab_lattice_get_bos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                MeCabNode { node: node }
            }
        }
    }

    /// Returns the end node of the sentence on success.
    fn get_eos_node(&self) -> MeCabNode {
        unsafe {
            let node = mecab_lattice_get_eos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                MeCabNode { node: node }
            }
        }
    }

    priv fn strerror(&self) -> ~str {
        unsafe {
            let s = mecab_lattice_strerror(self.lattice);
            raw::from_c_str(s)
        }
    }
}

/// The wrapper of `mecab::mecab_new` that may return `MeCab`.
pub fn new(args: &[~str]) -> MeCab {
    let argc = args.len() as c_int;

    let mut argptrs = ~[];
    let mut tmps = ~[];

    for args.each |arg| {
        let t = @copy *arg;
        tmps.push(t);
        argptrs.push(str::as_c_str(*t, |b| b));
    }
    argptrs.push(ptr::null());

    let mecab = vec::as_imm_buf(argptrs, |argv, _len| unsafe {
        mecab_new(argc, argv)
    });

    if mecab.is_null() {
        fail!(~"failed to create new instance");
    } else {
        MeCab { mecab: mecab }
    }
}

/// The wrapper of `mecab::mecab_new2` that may return `MeCab`.
pub fn new2(arg: &str) -> MeCab {
    let mecab = str::as_c_str(arg, |buf| unsafe {
        mecab_new2(buf)
    });

    if mecab.is_null() {
        fail!(~"failed to create new instance");
    } else {
        MeCab { mecab: mecab }
    }
}

/**
The wrapper of `mecab::mecab_model_new` that
may return uniquely managed `MeCabModel`.
*/
pub fn model_new(args: &[~str]) -> ~MeCabModel {
    let argc = args.len() as c_int;

    let mut argptrs = ~[];
    let mut tmps = ~[];

    for args.each |arg| {
        let t = @copy *arg;
        tmps.push(t);
        argptrs.push(str::as_c_str(*t, |b| b));
    }
    argptrs.push(ptr::null());

    let model = vec::as_imm_buf(argptrs, |argv, _len| unsafe {
        mecab_model_new(argc, argv)
    });

    if model.is_null() {
        fail!(~"failed to create new Model");
    } else {
        ~MeCabModel { model: model }
    }
}

/**
The wrapper of `mecab::mecab_model_new2` that
may return uniquely managed `MeCabModel`.
*/
pub fn model_new2(arg: &str) -> ~MeCabModel {
    let model = str::as_c_str(arg, |buf| unsafe {
        mecab_model_new2(buf)
    });

    if model.is_null() {
        fail!(~"failed to create new Model");
    } else {
        ~MeCabModel { model: model }
    }
}

/**
The wrapper of `mecab::mecab_version` that
returns version-number string.
*/
pub fn version() -> ~str {
    unsafe {
        let vers = mecab_version();
        raw::from_c_str(vers)
    }
}
