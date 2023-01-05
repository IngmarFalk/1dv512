use crate::shared::cq::{MessageQueue, Mq, Reader};
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

pub fn task3() {
    let chrs = vec!['A', 'B', 'C'];
    let mq = Arc::new(Mutex::new(Mq::new()));
    let mut handlers = vec![];

    let _cp = Arc::clone(&mq);
    let rx_thread = std::thread::spawn(move || {
        println!("Reading Thread.");
        loop {
            let mut rx = Reader::new(_cp.lock().unwrap());
            let msg = rx.recv();
            if let Some(msg) = msg {
                print!("{} ", msg);
                std::io::stdout().flush().unwrap();
            }
        }
    });

    for c in chrs {
        let _mq = Arc::clone(&mq);
        handlers.push(std::thread::spawn(move || loop {
            _mq.lock().unwrap().send(c);
        }));
    }
    handlers.push(rx_thread);

    for h in handlers {
        h.join().unwrap();
    }
}
