/*

test:

  % rustc mecab.rc
  % ./mecab test.txt

*/

fn test_pass_mecab(_mecab: mecab) {
}

fn test_mecab_new(args: [str]) {
    let m = mecab_new(vec::len(args), args);
    test_pass_mecab(m);

    let input = "夕焼け小焼けの赤とんぼ";
    let output = m.sparse_tostr(input);

    alt output {
        some::<str>(s) {
            std::io::print(#fmt["input: %s\n", input]);
            std::io::print(#fmt["output:\n%s", s]);

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
        none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
        }
    }
}

fn main(args: [str]) {
    alt task::try {||
        test_mecab_new(args);
    } {
        result::ok(())  { /* do nothing */ }
        result::err(()) { sys::set_exit_status(1); }
    }
    // let _res = mecab_do(vec::len(args), args);
}
