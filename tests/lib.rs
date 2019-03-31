struct MyList<T> where T: FnMut() -> () {
    arr: Vec<Holder<T>>
}

enum Holder<T> where T: FnMut() -> () {
    Func(Box<T>, u8),
    Elem(u8),
}

impl<T> Holder<T> where T: FnMut() -> () {
    fn run(&mut self) {
        if let Holder::Func(func, handle) = self {
            (func)();
        }
    }
}

#[test]
fn static_test() {
    use std::io::Write;

    let mut buffer = Vec::new();
    let mut arr = MyList {
        arr: vec![Holder::Func(Box::new(|| {
            write!(&mut buffer, "douleyei!")
            .expect("Buffer unwritable");
        }), 42)]
    };
    arr.arr[0].run();
    println!("{}", match &arr.arr[0] {
        Holder::Elem(elem) => elem.to_string(),
        Holder::Func(func, handle) =>
            String::from_utf8(buffer.clone())
                    .expect("Invalid buffer"),
    });
    assert_eq!(buffer, b"douleyei!");
}

//#[test]
//fn dynamic_test() {
//    use std::io::Write;
//    use std::rc::Rc;
//    use std::cell::RefCell;
//
//    extern crate event_loop;
//    use event_loop::EventLoop;
//
//    let mut result = Vec::new();
//    let mut event_loop = EventLoop::new();
//    event_loop.push(Box::new(|| {
//            write!(&mut result, "Been there, done that");
//        }), 42
//    );
//    event_loop.run();
//    assert_eq!(result, b"Been there, done that");
//}
