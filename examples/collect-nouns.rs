extern mod std;
extern mod mecab;

use std::arc;

use mecab::MeCabLattice;
use mecab::NOR_NODE;
use mecab::UNK_NODE;

fn collect_nouns(lattice: &MeCabLattice) -> ~[~str] {
    let mut v = ~[];

    let node = lattice.get_bos_node().get();
    for node.each |n| {
        let status = n.get_status();

        if status == NOR_NODE || status == UNK_NODE {
            let feature = n.get_feature();
            if str::eq_slice(feature.split_str(",")[0], "名詞") {
                v.push(n.get_surface());
            }
        }
    }

    move v
}

fn run_collector(collector: &pipes::PortSet<~[~str]>, n: uint) -> ~[~str] {
    let mut n = n;
    let mut nouns = ~[];
    while n > 0 {
        match collector.try_recv() {
            Some(move v) => nouns.push_all_move(move v),
            None => n -= 1
        }
    }
    move nouns
}

fn main() {
    let sentences = [
        "これはテストです",
        "太郎は次郎が持っている本を花子に渡した",
        "昨日の夕食はカレーでした",
    ];

    let model = result::unwrap(mecab::model_new2(""));
    let model = ~arc::ARC(move model);

    let collector = &pipes::PortSet();

    for sentences.each |sentence| {
        let sentence = *sentence;
        let model = ~arc::clone(model);

        let (c, p) = pipes::stream();
        collector.add(move p);

        do task::spawn |move model, move c| {
            let model = arc::get(model);
            let tagger = model.create_tagger().get();
            let lattice = model.create_lattice().get();

            lattice.set_sentence(sentence);

            if tagger.parse_lattice(lattice) {
                c.send(collect_nouns(lattice));
            }
        }
    }

    let nouns = run_collector(collector, sentences.len());
    for nouns.each |noun| {
        io::println(fmt!("noun: %s", *noun));
    }
}
