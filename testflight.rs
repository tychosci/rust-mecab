/*

test:

  % rustc mecab.rc
  % rustc testflight.rs -L .
  % ./testflight

*/

use std;
use mecab;

import mecab::{MECAB_UNK_NODE, MECAB_NOR_NODE};

fn test_pass_mecab(_mecab: mecab::mecab) {
}

fn example_singlethread(args: [str]) {
    let m = alt mecab::mecab_new(vec::len(args), args) {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");

    let input = "あなたの家から、あの森まで";
    let output = m.sparse_tostr(input);

    alt output {
      some::<str>(s) {
        std::io::print(#fmt["input: %s\n", input]);
        std::io::print(#fmt["output:\n%s", s]);
      }
      none::<str> {
        fail #fmt["Exception: %s", m.strerror()];
      }
    }
}

fn example_singlethread_use2() {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");

    let input = "抵抗は無意味だ";
    let output = m.sparse_tostr2(input, str::byte_len(input));

    alt output {
      some::<str>(s) {
        std::io::print(#fmt["input: %s\n", input]);
        std::io::print(#fmt["output:\n%s", s]);
      }
      none::<str> {
        fail #fmt["Exception: %s", m.strerror()];
      }
    }
}

fn example_mecab_node() {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");

    let input = "みなさんのおかげでチョー助かってます(>_<;)";

    let node = m.sparse_tonode(input);
    std::io::print(#fmt["input: %s\n", input]);

    alt node {
      some::<mecab::mecab_node>(n) {
        std::io::print("output:\n");
        while !n.is_end() {
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                std::io::print(#fmt["%s", n.get_surface()]);
                std::io::print(#fmt["\t%s\n", n.get_feature()]);
            }
            n.bump();
        }
      }
      node::<mecab::mecab_node> {
        fail #fmt["Exception: %s", m.strerror()];
      }
    }
}

fn example_mecab_dict() {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");

    let dict = alt m.get_dictionary_info() {
      some::<mecab::mecab_dictionary_info>(_dict) { _dict }
      none::<mecab::mecab_dictionary_info>        { fail; }
    };

    dict.iter { |d|
        std::io::print(#fmt["filename: %s\n", d.get_filename()]);
        std::io::print(#fmt["charset:  %s\n", d.get_charset()]);
        std::io::print(#fmt["size:     %u\n", d.get_size()]);
        std::io::print(#fmt["type:     %d\n", d.get_type()]);
        std::io::print(#fmt["lsize:    %u\n", d.get_lsize()]);
        std::io::print(#fmt["rsize:    %u\n", d.get_rsize()]);
        std::io::print(#fmt["version:  %u\n", d.get_version()]);
    }
}

fn example_katakanize() {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");
    std::io::println("[katakanize]");

    let input = "我々は宇宙人だ";
    std::io::print(#fmt["from: %s\n", input]);

    let node = m.sparse_tonode(input);

    alt node {
      some::<mecab::mecab_node>(n) {
        std::io::print("to:   ");
        while !n.is_end() {
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                let feature = n.get_feature();
                let kana = str::split_str(feature, ",")[7];
                std::io::print(#fmt["%s", kana]);
            }
            n.bump();
        }
        std::io::print("\n");
      }
      none::<mecab::mecab_node> {
        fail #fmt["Exception: %s", m.strerror()];
      }
    }
}

fn example_hinsi() {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    std::io::println("-----------------------------------------");
    std::io::println("[hinsi]");

    let input = "今日はやけに冷えるなあ";
    std::io::print(#fmt["input: %s\n", input]);

    let node = m.sparse_tonode(input);

    alt node {
      some::<mecab::mecab_node>(node0) {
        node0.iter { |n|
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                let feature = n.get_feature();
                let feature0 = str::split_str(feature, ",");
                let (a, b) = (feature0[0], feature0[6]);
                std::io::println(#fmt[" -> %s(%s)", b, a]);
            }
        }
      }
      none::<mecab::mecab_node> {
        fail #fmt["Exception: %s", m.strerror()];
      }
    }
}

fn main(args: [str]) {
    std::io::print(#fmt["version: %s\n", mecab::mecab_version()]);
    alt task::try {||
        example_singlethread(args);
        example_singlethread_use2();
        example_mecab_node();
        example_mecab_dict();
        example_katakanize();
        example_hinsi();
    } {
        result::ok(())  { /* do nothing */ }
        result::err(()) { sys::set_exit_status(1); }
    }
    // let _res = mecab_do(vec::len(args), args);
}
