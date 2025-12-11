use crate::screen::Screen;

mod screen;
mod vga;

fn main() {
    let mut s = Screen::default();

    for _ in 0..100 {
        s.push(0);
    }

    // *s.into_iter().skip(4).next().unwrap() = 1;

    for component in s.into_iter() {
        print!("{:?} ", component);
    }

    println!("");
    println!("");

    for line in s.lines() {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }

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
