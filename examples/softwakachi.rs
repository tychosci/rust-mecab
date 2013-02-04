extern mod mecab;

fn main() {
    let mecab = mecab::new2("-m -a");
    let node = mecab.parse_to_node("東京特許許可局");
    for node.each |&node| {
        let prob = node.get_prob() as float;
        let is_normal_node = node.get_status() == mecab::NOR_NODE;
        if is_normal_node && (node.is_best() || prob >= 0.05) {
            let surface = node.get_surface();
            let feature = node.get_feature();
            io::println(fmt!("%s\t%s\t%f", surface, feature, prob));
        }
    }
}
