use super::*;

#[test]
fn test_mecab_version() {
    let vers = version();
    assert!(!vers.is_empty());
}

#[test]
fn test_mecab_dictionary_info() {
    let mecab = new2("");
    let dict  = mecab.get_dictionary_info();

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

#[test]
fn test_mecab_parse() {
    let mecab = new2("");
    let s = mecab.parse("この文はテストです");
    io::println(fmt!("%s", s));
}

#[test]
fn test_mecab_parse_to_node() {
    let mecab = new2("");
    let node = mecab.parse_to_node("この文はテストです");

    for node.each |n| {
        let status = n.get_status();
        if status == NOR_NODE || status == UNK_NODE {
            io::println(fmt!("surface: %s", n.get_surface()));
        }
    }
}
