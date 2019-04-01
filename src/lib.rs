//! A crate that manages an event loop

pub struct FuncBox<T> where T: FnMut() -> () {
    pub func: Box<T>,
    pub handle: u8,
}

pub struct EventLoop<T> where T: FnMut() -> () {
    funcs: Vec<FuncBox<T>>,
    cur: usize
}

impl<T> FuncBox<T> where T: FnMut() -> () {
    fn run(&mut self) {
        (self.func)();
    }
}

#[test]
fn runs() {
    use std::io::Write;

    let mut buffer = Vec::new();
    let mut test_box = FuncBox { func: Box::new(|| {
        write!(&mut buffer, "Working great!")
        .expect("Could not write to buffer");
    }), handle: 42 };

    test_box.run();

    assert_eq!(test_box.handle, 42);
    assert_eq!(buffer, b"Working great!");
}

impl<T> EventLoop<T> where T: FnMut() -> () {
    pub fn new() -> EventLoop<T> {
        EventLoop {
            funcs: Vec::new(),
            cur: 0
        }
    }

    pub fn push(&mut self, func: Box<T>, handle: u8) {
        self.funcs.push(FuncBox{ func, handle });
    }

    //pub fn remove(&mut self, handle &'a str) {
    //   self.funcs
    //}

    pub fn next(&mut self) -> bool {
        self.funcs[self.cur].run();
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
