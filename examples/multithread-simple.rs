extern mod std;
extern mod mecab;

use std::arc;

fn main() {
    let s = "これはテストです";
    let model = result::unwrap(mecab::model_new2(""));
    let model = ~arc::ARC(move model);

    for 2.times {
        let model = ~arc::clone(model);

        do task::spawn |move model| {
            let model = arc::get(model);
            let tagger = model.create_tagger().get();
            let lattice = model.create_lattice().get();

            lattice.set_sentence(s);

            if tagger.parse_lattice(lattice) {
                io::println("result: ");
                io::println(fmt!("%s", lattice.to_str()));
            }
        }
    }

    let model = arc::get(model);
    let tagger = model.create_tagger().get();
    let lattice = model.create_lattice().get();

    lattice.set_sentence(s);

    if tagger.parse_lattice(lattice) {
        io::println("result: ");
        io::println(fmt!("%s", lattice.to_str()));
    }
}
