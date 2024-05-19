use console::Term;
use rand::{distributions::Uniform, Rng};
use std::{
    io::{stdout, Write},
    string,
};
use term_size;
use array2d::Array2D;
use std::thread;
use std::time::Duration;

const CHAR_SET: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
#[derive(Debug)]
struct MtxSubLine {
    current_head_pos: i32,
    length: usize,
    chars: Vec<char>,
}
impl MtxSubLine {
    fn new(
        length: usize,
        current_head_pos: i32,
        chars_idx: &Vec<char>,
    ) -> Self {
        MtxSubLine {
            length,
            current_head_pos,
            chars: chars_idx.clone(),
        }
    }
}
#[derive(Debug)]
struct MtxLine {
    lines: Vec<MtxSubLine>,
}
#[derive(Debug)]
struct Mtx {
    lines: Vec<MtxLine>,
}
fn generate_subline(head_pos: i32) -> MtxSubLine {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(2..6);
    let offset = rng.gen_range(1..6);
    let subline_chars: Vec<char> = (&mut rng)
        .sample_iter(Uniform::new_inclusive(0, 4))
        .take(len)
        .map(|idx| CHAR_SET[idx])
        .collect();
    MtxSubLine::new(len, head_pos-offset, &subline_chars)
}
fn display2(mtx: &Mtx) {
    let term = Term::stdout();
    let _ = term.clear_screen();
    let (term_w, term_h) = term_size::dimensions().unwrap();
    let mut screen = Array2D::filled_with(' ', term_h, term_w);

    let mut column_index: usize = 0;
    for line in mtx.lines.iter().take(term_w).collect::<Vec<&MtxLine>>() {
        for sub_line in &line.lines {
            let mut row_index=sub_line.current_head_pos;
            for c in &sub_line.chars {
                if row_index<0 { break; }
                screen[(row_index as usize, column_index)]= *c;
                row_index -= 1;
            }
        }
        for row in screen.rows_iter() {
            println!("{}", row.collect::<String>());
        }
        column_index += 1;
    }
}
fn is_subline_fit(subline: &MtxSubLine, term_h: i32)->bool {
   (subline.current_head_pos - subline.length as i32) >= term_h
}
fn move_down(mtx: &mut Mtx, term_h: usize){
    mtx.lines.iter_mut().for_each(|line| {
        line.lines.iter_mut().for_each(|subline| { subline.current_head_pos +=1; });
        if line.lines.iter()
            .filter(|subline| {is_subline_fit(subline, term_h as i32)})
            .count() > 0 {
                line.lines.retain(|subline| {is_subline_fit(subline, term_h as i32)});
                line.lines.push(generate_subline(0));
            }

      //  line.lines.pu
    })
}

fn main() {
    //println!("{:?}", subline_chars);
    //let line = MtxLine { lines: vec![&mut subline] };
    let (term_w, term_h) = term_size::dimensions().unwrap();
    let mut mtx = Mtx { lines: vec![] };

    mtx.lines.resize_with(term_w, || {
        let mut mtx_sub_lines: Vec<MtxSubLine> = vec![];
        let mut curent_head_pos: i32 = term_h as i32;
        loop {
            let sub_line = generate_subline(curent_head_pos);
            curent_head_pos = sub_line.current_head_pos - sub_line.length as i32;
            if curent_head_pos > 0 {
                mtx_sub_lines.push(sub_line);
            } else {break;}
        }
        MtxLine {
            lines: mtx_sub_lines,
        }
    });
   display2(&mtx);
   /*
   thread::sleep(Duration::from_millis(2000)); 
   move_down(&mut mtx, term_h);
   
   display2(&mtx);
   thread::sleep(Duration::from_millis(2000)); 
   move_down(&mut mtx, term_h);
   display2(&mtx);
   */
    //println!("Terminal width {} terminal height {}", term_w, term_h );
    //println!("{:?}", mtx.lines.get(0).unwrap());
}
