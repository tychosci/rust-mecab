// test:
// % rustc mecab.rc
// % ./mecab test.txt

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
        }
        none::<str> {
            fail #fmt["Exception: %s", m.strerror()];
        }
    }
}

fn main(args: [str]) {
    alt task::try {||
        test_mecab_new(args);
        true
    } {
        result::ok(_) { /* do nothing */ }
        result::err(_) { sys::set_exit_status(1); }
    }
    // let _res = mecab_do(vec::len(args), args);
}
