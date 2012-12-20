extern mod mecab;

fn main() {
    let mecab = mecab::new2("").unwrap();

    let input = "うらにわにはにわにわにはにわにわとりがいる";

    io::println(fmt!("input: %s", input));

    let node = mecab.parse_to_node(input).unwrap();

    io::print("output: ");

    for node.each |n| {
        let status = n.get_status();

        if status == mecab::UNK_NODE || status == mecab::NOR_NODE {
            io::print(fmt!("%s ", n.get_surface()));
        }
    }

    io::print("\n");
}
