extern mod mecab;

use mecab::IMeCabNode;

fn main() {
    let mecab = mecab::new2("");

    let input = "我々は、宇宙人だ";

    io::println(fmt!("input: %s", input));

    let node = mecab.parse_to_node(input);

    io::print("output: ");

    for node.each |n| {
        let status = n.get_status();

        if status == mecab::NOR_NODE {
            let mut i = 0;
            let feature = n.get_feature();
            for feature.each_split_char(',') |s| {
                if i == 7 { io::print(fmt!("%s", s)); }
                i += 1;
            }
        }
    }

    io::print("\n");
}
