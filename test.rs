#[test]
fn test_mecab_version() {
    let vers_str = mecab_version();
    assert vers_str.is_not_empty();
}

#[test]
fn test_mecab_new() {
    let status = match mecab_new(["test_mecab_new"]) {
        some(_) => true,
        none    => false,
    };
    assert status;
}

#[test]
fn test_mecab_new2() {
    let status = match mecab_new2("") {
        some(_) => true,
        none    => false,
    };
    assert status;
}

#[test]
fn test_mecab_dictionary_info() {
    let mecab = mecab_new2("").get();
    let dict  = mecab.get_dictionary_info().get();

    for dict.each |d| {
        io::println(fmt!("filename: %s", d.get_filename()));
        io::println(fmt!("charset:  %s", d.get_charset()));
        io::println(fmt!("size:     %?", d.get_size()));
        io::println(fmt!("type:     %?", d.get_type()));
        io::println(fmt!("lsize:    %?", d.get_lsize()));
        io::println(fmt!("rsize:    %?", d.get_rsize()));
        io::println(fmt!("version:  %?", d.get_version()));
    }
}
