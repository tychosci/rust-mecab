// test:
// % rustc mecab.rc
// % ./mecab test.txt

fn main(args: [str]) {
    let _res = mecab_do(vec::len(args), args);
}
