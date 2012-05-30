/*

test:

  % rustc mecab.rc
  % rustc testflight.rs -L .
  % ./testflight

*/

use std;
use mecab;

import io::{print, println};
import mecab::{MECAB_UNK_NODE, MECAB_NOR_NODE};

fn test_pass_mecab(_mecab: mecab::mecab) {
}

fn example_do(title: str, blk: fn(mecab::mecab)) {
    let m = alt mecab::mecab_new2("") {
      some::<mecab::mecab>(_m) { _m }
      none::<mecab::mecab>     { fail; }
    };

    test_pass_mecab(m);

    print("\n");
    println("-----------------------------------------");
    println(#fmt["[%s]", title]);
    println("-----------------------------------------");

    blk(m);
}

fn example_singlethread() {
    example_do("singlethread") { |m|
        let input = "あなたの家から、あの森まで";

        alt m.sparse_tostr(input) {
          some::<str>(s) {
            print(#fmt["input: %s\n", input]);
            print(#fmt["output:\n%s", s]);
          }
          none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        }

        alt m.nbest_sparse_tostr(3u, input) {
          some::<str>(s) {
            print(#fmt["input: %s\n", input]);
            print(#fmt["nbest_output:\n%s", s]);
          }
          none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        }
    }
}

fn example_singlethread_use2() {
    example_do("singlethread_use2") { |m|
        let input = "抵抗は無意味だ";
        let output = m.sparse_tostr2(input, str::len(input));

        alt output {
          some::<str>(s) {
            print(#fmt["input: %s\n", input]);
            print(#fmt["output:\n%s", s]);
          }
          none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        }
    }
}

fn example_mecab_node() {
    example_do("mecab_node") { |m|
        let input = "みなさんのおかげでチョー助かってます(>_<;)";
        print(#fmt["input: %s\n", input]);

        let node = alt m.sparse_tonode(input) {
          some::<mecab::mecab_node>(n) { n }
          none::<mecab::mecab_node> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        };

        print("output:\n");

        node.iter { |n|
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                print(#fmt["%s", n.get_surface()]);
                print(#fmt["\t%s\n", n.get_feature()]);
            }
        }
    }
}

fn example_mecab_dict() {
    example_do("mecab_dict") { |m|
        let dict = alt m.get_dictionary_info() {
          some::<mecab::mecab_dictionary_info>(_dict) { _dict }
          none::<mecab::mecab_dictionary_info>        { fail; }
        };

        dict.iter { |d|
            print(#fmt["filename: %s\n", d.get_filename()]);
            print(#fmt["charset:  %s\n", d.get_charset()]);
            print(#fmt["size:     %u\n", d.get_size()]);
            print(#fmt["type:     %d\n", d.get_type()]);
            print(#fmt["lsize:    %u\n", d.get_lsize()]);
            print(#fmt["rsize:    %u\n", d.get_rsize()]);
            print(#fmt["version:  %u\n", d.get_version()]);
        }
    }
}

fn example_katakanize() {
    example_do("katakanize") { |m|
        let input = "我々は宇宙人だ";
        print(#fmt["from: %s\n", input]);

        let node = alt m.sparse_tonode(input) {
          some::<mecab::mecab_node>(n) { n }
          none::<mecab::mecab_node> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        };

        print("to:   ");

        node.iter { |n|
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                let feature = n.get_feature();
                let kana = str::split_str(feature, ",")[7];
                print(#fmt["%s", kana]);
            }
        }

        print("\n");
    }
}

fn example_hinsi() {
    example_do("hinsi") { |m|
        let input = "今日はやけに冷えるなあ";
        print(#fmt["input: %s\n", input]);

        let node = alt m.sparse_tonode(input) {
          some::<mecab::mecab_node>(n) { n }
          none::<mecab::mecab_node> {
            fail #fmt["Exception: %s", m.strerror()];
          }
        };

        node.iter { |n|
            let stat = n.get_status();
            if stat == MECAB_NOR_NODE || stat == MECAB_UNK_NODE {
                let feature = n.get_feature();
                let feature0 = str::split_str(feature, ",");
                let (a, b) = (feature0[0], feature0[6]);
                println(#fmt[" -> %s(%s)", b, a]);
            }
        }
    }
}

fn example_nbest_iter() {
    example_do("nbest_iter") { |m|
        let input = "すもももももももものうち";
        print(#fmt["input: %s\n", input]);

        if !m.nbest_init(input) {
            fail #fmt["Exception: %s", m.strerror()];
        }

        m.nbest_upto(3u) { |m0|
            let s = alt m0.nbest_next_tostr() {
                some::<str>(_s) { _s }
                none::<str>     { fail; }
            };
            print(#fmt["output:\n%s", s]);
        }
    }
}

fn main() {
    print(#fmt["version: %s\n", mecab::mecab_version()]);
    alt task::try {||
        example_singlethread();
        example_singlethread_use2();
        example_mecab_node();
        example_mecab_dict();
        example_katakanize();
        example_hinsi();
        example_nbest_iter();
    } {
        result::ok(())  { /* do nothing */ }
        result::err(()) { os::set_exit_status(1); }
    }
}
