use crate::{entry::Entry, screen::Screen, vga::Buffer};

mod entry;
mod screen;
mod vga;

fn main() {
    let mut s = Screen::default();
    //
    // for i in 0..23 {
    //     s.push(Entry::new(b'a'));
    // }
    //
    // for e in s.into_iter().rev() {
    //     println!("{}", e);
    // }
    //
    // return;
    // *s.into_iter().skip(4).next().unwrap() = 1;
    //
    // println!("");
    // println!("");
    //
    // for line in s.lines().skip(1) {
    //     for c in line {
    //         print!("{}", c);
    //     }
    //     println!("");
    // }
    //
    // let buffer = Buffer::from_screen(&mut s);
    // for l in buffer.entries.iter() {
    //     println!("{:?}", l);
    // }

    // let mut p = Screen::default();

    // for i in 0..=21 {
    //     p.push(i as u16);
    //     println!("{:?}", p.entries);
    //     println!("{:?}", p.head);
    //     println!();
    // }
    //
    // for e in p.into_iter().take(9) {
    //     *e = 42;
    // }
    //
    // for component in p.into_iter() {
    //     print!("{} ", component);
    // }
    // println!();

    // for _ in 0..20 {
    //     match p.remove_last() {
    //         Some(x) => println!("removed {}", x),
    //         None => println!("not removed"),
    //     }
    //     for component in p.into_iter() {
    //         print!("{} ", component);
    //     }
    //     println!();
    // }
}
