use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

use crate::shared::semaphore::Semaphore;

pub fn task1() {
    let mut c = vec![];
    let s = Arc::new(Mutex::new(1));

    // spawn Thread 1
    //   if 1 == 1 => true => while ( sleep )
    // spawn Thread 2
    //   if 0 == 1 => false => continue
    //   print `A`
    //   update s to 0
    // in Thread 1
    //   if 0 == 1 => false => continue
    //   print `B`
    //   update s to 1
    // ...
    // repeat 10 times

    for i in 0..2 {
        let _s = Arc::clone(&s);
        c.push(thread::spawn(move || {
            for _ in 0..10 {
                while *_s.lock().unwrap() == i {
                    // spin
                }

                print!("{}", (b'A' + i) as char);
                *_s.lock().unwrap() = i;
            }
        }));
    }

    for t in c {
        t.join().unwrap();
    }

    // let s = Arc::new(Semaphore::binary());
}
