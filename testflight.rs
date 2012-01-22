// test:
// % rustc mecab.rc
// % ./mecab test.txt

fn test_mecab_new(args: [str]) {
    let m = mecab_new(vec::len(args), args);
    let input = "夕焼け小焼けの赤とんぼ";
    let output = m.sparse_tostr(input);
    std::io::println(#fmt["input: %s\n", input]);
    std::io::println(#fmt["output:\n%s", output]);
}

fn main(args: [str]) {
    test_mecab_new(args);
    // let _res = mecab_do(vec::len(args), args);
}
