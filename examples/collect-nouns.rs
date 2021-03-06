extern mod std;
extern mod mecab;

use std::arc;

use mecab::IMeCabNode;
use mecab::MeCabLattice;
use mecab::NOR_NODE;
use mecab::UNK_NODE;

fn collect_nouns(lattice: &MeCabLattice) -> ~[~str] {
    let mut v = ~[];

    let node = lattice.get_bos_node();
    for node.each |n| {
        let status = n.get_status();

        if status == NOR_NODE || status == UNK_NODE {
            let feature = n.get_feature();
            for feature.each_split_char(',') |s| {
                if s == "名詞" { v.push(n.get_surface()); }
                break;
            }
        }
    }
    return v;
}

fn main() {
    let sentences = [
        "これはテストです",
        "太郎は次郎が持っている本を花子に渡した",
        "昨日の夕食はカレーでした",
    ];

    let model = mecab::model_new2("");
    let model = ~arc::ARC(model);

    let (p, c) = comm::stream();
    let c = comm::SharedChan(c);

    for sentences.each |&sentence| {
        let model = ~arc::clone(model);
        let c = c.clone();

        do task::spawn_supervised {
            let model = arc::get(model);
            let tagger = model.create_tagger();
            let lattice = model.create_lattice();

            lattice.set_sentence(sentence);

            if tagger.parse_lattice(&lattice) {
                c.send(collect_nouns(&lattice));
            }
        }
    }

    for sentences.len().times {
        let nouns = p.recv();
        for nouns.each |noun| {
            io::println(fmt!("noun: %s", *noun));
        }
    }
}
