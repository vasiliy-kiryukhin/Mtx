use rand::Rng;
const CHAR_SET: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
const DEFAULT_CHAR : char = 'Z';

struct MtxSubLine {
    length: u8,
    chars: Vec<char>,
    top_position: i32,
    bottom_position: i32,
}
impl MtxSubLine {
    fn new(length: u8, bottom_position: i32, chars_idx: &Vec<usize>) -> Self {
        let subline_chars: Vec<char> = chars_idx.iter().map(|x| *CHAR_SET.get(*x).unwrap()).collect();
        MtxSubLine {
            length,
            chars: subline_chars,
            top_position: bottom_position + length as i32,
            bottom_position,
        }
    }
    fn move_down(&mut self) {
        self.bottom_position +=1;
        self.top_position +=1;
    }
}
struct MtxLine<'a> {
    lines: Vec<&'a mut MtxSubLine>,
}
fn main() {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(5..10);
    let chars_idx : Vec<usize> = (0..3).map(|_| rng.gen_range(0..4)).collect();

    let subLine = MtxSubLine::new(len, -3, &chars);
    let line = MtxLine { lines: vec![] };
}
