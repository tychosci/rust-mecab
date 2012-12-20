extern mod mecab;

fn main() {
    let mecab = mecab::new2("").unwrap();

    let input = "我々は、宇宙人だ";

    io::println(fmt!("input: %s", input));

    let node = mecab.parse_to_node(input).unwrap();

    io::print("output: ");

    for node.each |n| {
        let status = n.get_status();

        if status == mecab::NOR_NODE {
            let feature = n.get_feature();
            io::print(fmt!("%s", feature.split_str(",")[7]));
        }
    }

    io::print("\n");
}
