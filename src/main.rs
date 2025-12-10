const BUFFER_SIZE: usize = 10;
struct Screen {
    entries: [u16; BUFFER_SIZE],
    head: usize,
    len: usize,
}

impl Screen {
    pub fn push(&mut self, e: u16) {
        if self.entries.len() <= self.len {
            self.entries[self.head] = e;
            self.head += 1;
        } else {
            self.entries[(self.head + self.len) % self.entries.len()] = e;
            self.len += 1;
        }
    }

    pub fn remove_last(&mut self) -> Option<u16> {
        if self.len == 0 {
            None
        } else {
            let idx = (self.head + self.len - 1) % self.entries.len();
            let e = self.entries[idx];
            self.len -= 1;
            Some(e)
        }
    }
}

impl<'a> IntoIterator for &'a Screen {
    type Item = &'a u16;
    type IntoIter = ScreenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ScreenIterator {
            pixel: self,
            index: 0,
        }
    }
}

pub struct ScreenIterator<'a> {
    pixel: &'a Screen,
    index: usize,
}

impl<'a> Iterator for ScreenIterator<'a> {
    type Item = &'a u16;
    fn next(&mut self) -> Option<&'a u16> {
        let next = self.pixel.entries[self.pixel.head..]
            .iter()
            .chain(self.pixel.entries[..self.pixel.head].iter())
            .take(self.pixel.len)
            .skip(self.index)
            .next();
        self.index += 1;
        next
    }
}
fn main() {
    let mut p = Screen {
        entries: [0; BUFFER_SIZE],
        head: 0,
        len: 0,
    };
    for i in 0..20 {
        p.push(i as u16);
        for component in p.into_iter() {
            print!("{} ", component);
        }
        println!();
    }
    for _ in 0..20 {
        match p.remove_last() {
            Some(x) => println!("removed {}", x),
            None => println!("not removed"),
        }
        for component in p.into_iter() {
            print!("{} ", component);
        }
        println!();
    }
}
