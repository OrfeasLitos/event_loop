//! A crate that manages an event loop

pub mod event_loop {
    pub struct FuncBox<'a> {
        pub func: Box<FnMut() -> ()>,
        pub handle: &'a str
    }

    pub struct EventLoop<'a> {
        funcs: Vec<FuncBox<'a>>,
        cur: usize
    }

    impl<'a> EventLoop<'a> {
        pub fn new() -> EventLoop<'a> {
            EventLoop {
                funcs: Vec::new(),
                cur: 0
            }
        }

        pub fn push(&mut self, func: Box<FnMut() -> ()>, handle: &'a str) {
            self.funcs.push(FuncBox{ func, handle });
        }

        //pub fn remove(&mut self, handle &'a str) {
        //   self.funcs
        //}

        pub fn next(&mut self) -> bool {
            (&mut self.funcs[self.cur].func)();
            self.cur = (self.cur + 1) % self.funcs.len();
            self.cur == 0
        }

        pub fn run(&mut self) {
            let mut completed = false;
            while !completed {
                completed = self.next();
            }
        }
    }
}

#[test]
fn test() {
    use std::io::Write;
    use std::rc::Rc;
    use std::cell::RefCell;

    use self::event_loop::EventLoop;

    let result /*: Rc<RefCell<Vec<u8>>>*/ = Rc::new(RefCell::new(Vec::new()));
    let mut event_loop = EventLoop::new();
    event_loop.push(Box::new(|| {
            writeln!(*result.borrow_mut(), "Been there, done that");
        }), "handl"
    );
    event_loop.run();
    assert_eq!(*result.borrow(), b"Been there, done that");
}
