use rand::prelude::*;
use rand::Rng;
const CHAR_SET: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
struct MtxSubLine {
    length: u8,
    chars: Vec<char>,
    top_position: i32,
    bottom_position: i32,
}
impl MtxSubLine {
    fn new(length: u8, bottom_position: i32, chars_idx: &Vec<i32>) -> Self {
        MtxSubLine {
            length,
            chars: vec![],
            top_position: bottom_position + length as i32,
            bottom_position,
        }
    }
}
struct MtxLine<'a> {
    lines: Vec<&'a mut MtxSubLine>,
}
fn main() {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(5..10);

    let line = MtxLine { lines: vec![] };
}
