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

#[test]
fn static_test() {
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

//    #[test]
//    fn dynamic_test() {
//        use std::io::Write;
//        use std::rc::Rc;
//        use std::cell::RefCell;
//
//        use super::event_loop::EventLoop;
//
//        let result /*: Rc<RefCell<Vec<u8>>>*/ = Rc::new(RefCell::new(Vec::new()));
//        let mut event_loop = EventLoop::new();
//        event_loop.push(Box::new(|| {
//                writeln!(*result.borrow_mut(), "Been there, done that");
//            }), "handl"
//        );
//        event_loop.run();
//        assert_eq!(*result.borrow(), b"Been there, done that");
//    }
