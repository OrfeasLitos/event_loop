//! A crate that manages an event loop

pub struct FuncBox {
    pub func: Box<FnMut() -> ()>,
    pub handle: u8,
}

pub struct EventLoop {
    funcs: Vec<FuncBox>,
    cur: usize
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            funcs: Vec::new(),
            cur: 0
        }
    }

    pub fn push(&mut self, func: Box<FnMut() -> ()>, handle: u8) {
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
