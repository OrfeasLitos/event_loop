//! A crate that manages an event loop

struct FuncBox<'a> {
    func: Box<FnMut() -> ()>,
    handle: &'a str
}

struct EventLoop<'a> {
    funcs: Vec<FuncBox<'a>>,
    cur: usize
}

impl<'a> EventLoop<'a> {
    fn new() -> EventLoop<'a> {
        EventLoop {
            funcs: Vec::new(),
            cur: 0
        }
    }

    fn push(&mut self, func: Box<FnMut() -> ()>, handle: &'a str) {
        self.funcs.push(FuncBox{ func, handle });
    }

    fn next(&mut self) -> bool {
        (&mut self.funcs[self.cur].func)();
        self.cur = (self.cur + 1) % self.funcs.len();
        self.cur == 0
    }

    fn run(&mut self) {
        let mut completed = false;
        while !completed {
            completed = self.next();
        }
    }
}

#[test]
fn test() {
    use std::io::Write;
    use std::rc::Rc;
    use std::cell::RefCell;

    let mut event_loop = EventLoop::new();
    let result = Rc::new(RefCell::new(Vec::new()));
    event_loop.push(Box::new(|| {
            writeln!(*result.borrow_mut(), "Been there, done that");
        }), "handl"
    );
    event_loop.run();
    assert_eq!(*result.borrow(), b"Been there, done that");
}
