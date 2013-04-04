rust-mecab
==========

[MeCab](http://mecab.sourceforge.net/) bindings for Rust.

Installation
------------

    cargo install mecab

Note that you need to install `mecab` and `mecab-ipadic` from
source or package manager if you don't have installed it yet.

Examples
--------

### わかち書き

    extern mod mecab;

    use mecab::IMeCabNode;

    fn main() {
        let mecab = mecab::new2("");

        let input = "うらにわにはにわにわにはにわにわとりがいる";

        io::println(fmt!("input: %s", input));

        let node = mecab.parse_to_node(input);

        io::print("output: ");

        for node.each |n| {
            let status = n.get_status();

            if status == mecab::UNK_NODE || status == mecab::NOR_NODE {
                io::print(fmt!("%s ", n.get_surface()));
            }
        }

        io::print("\n");
    }

License
-------

Copyright (C) 2012 Tycho Sci.

This binding is licensed under the same license of MeCab.
