/*

test:

  % rustc mecab.rc
  % rustc testflight.rs -L .
  % ./testflight

*/

use std;
use mecab;

fn test_pass_mecab(_mecab: mecab::mecab) {
}

fn example_singlethread(args: [str]) {
    let m = mecab::mecab_new(vec::len(args), args);

    test_pass_mecab(m);

    let input = "夕焼け小焼けの赤とんぼ";
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

    let dict = m.get_dictionary_info();

    while !dict.is_end() {
        std::io::print(#fmt["filename: %s\n", dict.get_filename()]);
        std::io::print(#fmt["charset:  %s\n", dict.get_charset()]);
        std::io::print(#fmt["size:     %u\n", dict.get_size()]);
        std::io::print(#fmt["type:     %d\n", dict.get_type()]);
        std::io::print(#fmt["lsize:    %u\n", dict.get_lsize()]);
        std::io::print(#fmt["rsize:    %u\n", dict.get_rsize()]);
        std::io::print(#fmt["version:  %u\n", dict.get_version()]);
        dict.bump();
    }
}

fn example_singlethread_use2() {
    let m = mecab::mecab_new2("");

    test_pass_mecab(m);

    let input = "我々は宇宙人だ";
    let output = m.sparse_tostr2(input);

    alt output {
        some::<str>(s) {
            std::io::print(#fmt["input: %s\n", input]);
            std::io::print(#fmt["output:\n%s", s]);
        }
        none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
        }
    }

    let dict = m.get_dictionary_info();

    while !dict.is_end() {
        std::io::print(#fmt["filename: %s\n", dict.get_filename()]);
        std::io::print(#fmt["charset:  %s\n", dict.get_charset()]);
        std::io::print(#fmt["size:     %u\n", dict.get_size()]);
        std::io::print(#fmt["type:     %d\n", dict.get_type()]);
        std::io::print(#fmt["lsize:    %u\n", dict.get_lsize()]);
        std::io::print(#fmt["rsize:    %u\n", dict.get_rsize()]);
        std::io::print(#fmt["version:  %u\n", dict.get_version()]);
        dict.bump();
    }
}

fn main(args: [str]) {
    std::io::print(#fmt["version: %s\n", mecab::mecab_version()]);
    alt task::try {||
        example_singlethread(args);
        example_singlethread_use2();
    } {
        result::ok(())  { /* do nothing */ }
        result::err(()) { sys::set_exit_status(1); }
    }
    // let _res = mecab_do(vec::len(args), args);
}
