 rust-mecab
============

[MeCab](http://mecab.sourceforge.net/) bindings for Rust.

MeCab is a Japanese Morphological Analyzer which commonly used in things  
related to Natural Language Processing
(e.g. IME, Search Engines, Text-to-Speech, Speech Recognition).

 Installation
--------------

    cargo install mecab

**NOTE:** you need to install `mecab` and `mecab-ipadic` from source or package manager
if you don't have installed it yet.

 Examples
----------

### わかち書き

    use std;
    use mecab;

    import mecab::{MECAB_NOR_NODE, MECAB_UNK_NODE};

    fn main() {
        let m = option::get(mecab::mecab_new2(""));

        let input = "うらにわにはにわにわにはにわにわとりがいる";

        std::io::println(#fmt["input: %s", input]);

        let node = option::get(m.sparse_tonode(input));

        std::io::print("output: ");

        node.iter { |n|
            let stat = n.get_status();
            if stat == MECAB_UNK_NODE || stat == MECAB_NOR_NODE {
                let surface = n.get_surface();
                std::io::print(#fmt["%s ", surface]);
            }
        }

        std::io::print("\n");
    }


See [testflight.rs](https://github.com/tychosci/rust-mecab/blob/master/testflight.rs)
for more details.

 TODO
------

- `mecab_model_t`
- `mecab_lattice_t`
- documentation

 License
---------

Copyright (C) 2012 Tycho Sci.

This binding is licensed under the same license of MeCab.
