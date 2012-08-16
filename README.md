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

    use mecab;

    fn main() {
        let mecab = mecab::mecab_new2("").get();

        let input = "うらにわにはにわにわにはにわにわとりがいる";

        io::println(fmt!("input: %s", input));

        let node = mecab.parse_to_node(input).get();

        io::print("output: ");

        for node.each |n| {
            let status = n.get_status();

            if status == mecab::MECAB_UNK_NODE ||
               status == mecab::MECAB_NOR_NODE {
                io::print(fmt!("%s ", n.get_surface()));
            }
        }

        io::print("\n");
    }


TODO
----

- `mecab_model_t`
- `mecab_lattice_t`
- documentation

License
-------

Copyright (C) 2012 Tycho Sci.

This binding is licensed under the same license of MeCab.
