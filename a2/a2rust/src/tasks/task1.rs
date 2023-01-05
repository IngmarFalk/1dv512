use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::shared;

pub fn task1() {
    let mut handles = vec![];
    let sem = Arc::new(shared::sem::Sem::<1, i16>::from(1));

    for i in 0..2 {
        let _sem = Arc::clone(&sem);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                while _sem.is_turn(i as i16) {}

                print!("{}", (b'A' + i as u8) as char);

                if _sem.status() == 0 {
                    println!()
                }

                _sem.set(i);
            }
        }));
    }

    for t in handles {
        t.join().unwrap();
    }
}
