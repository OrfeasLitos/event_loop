extern crate event_loop;
use event_loop::EventLoop;

#[test]
fn dynamic_test() {
    use std::io::Write;

    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();
    let mut events = EventLoop::new();

    events.push(Box::new(|| {
            write!(&mut buffer1, "Things have happened...")
            .expect("Could not write to buffer1");
        }), 42
    );
    events.push(Box::new(|| {
            write!(&mut buffer2, "Been there, done that")
            .expect("Could not write to buffer2");
        }), 42
    );

    events.run();

    assert_eq!(buffer1, b"Things have happened...");
    assert_eq!(buffer2, b"Been there, done that");
}
