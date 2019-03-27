//! A crate that manages an event loop

struct FuncBox<'a> {
    func: Box<FnMut() -> ()>,
    name: &'a str
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

    fn push(&mut self, func: Box<FnMut() -> ()>, name: &'a str) {
        self.funcs.push(FuncBox{ func, name });
    }

    fn next(&mut self) -> bool {
        println!("{}", self.funcs[self.cur].name);
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
    //use std::io::Write;
    //use std::rc::Rc;

    let mut event_loop = EventLoop::new();
    //let mut result = Rc::new(Vec::new());
    event_loop.push(Box::new(|| {
            println!("Been there, done that");
        }), "functionn"
    );
    event_loop.run();
    //assert_eq!(result, b"Been there, done that");
}
