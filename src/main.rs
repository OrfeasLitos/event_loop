pub enum Holder<T> where T: FnMut() -> () {
    Func(Vec<(Box<T>, u8)>),
    Elem(Vec<u8>),
}

impl<T> Holder<T> where T: FnMut() -> () {
    fn run(&mut self, index : usize) {
        use std::ops::IndexMut;

        if let Holder::Func(vec) = self {
            (vec.index_mut(index).0)();
        }
    }

    pub fn bump(&mut self, index : usize) {
        if let Holder::Elem(vec) = self {
            vec[index] += 1;
        }
    }
}

fn main() {
    use std::io::Write;

    let mut buffer = Vec::new();
    let input = vec![(Box::new(|| {
        writeln!(&mut buffer, "douleyei!")
        .expect("Buffer unwritable");
    }), 42)];
    let mut holder = Holder::Func(input);
    holder.run(0);
    println!("{}", match holder {
        Holder::Elem(vec) => vec[0].to_string(),
        Holder::Func(_vec) => String::from_utf8(buffer)
                              .expect("Invalid buffer"),
    });
}
