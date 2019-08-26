use std::fs;

fn open_case_source() -> fs::File {
    fs::File::open("/home/t/git/myopic/data/formatted-three-puzzles").unwrap()
}

struct TestCase {

}