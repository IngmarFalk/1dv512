use std::{sync::Arc, thread};

use crate::shared;

pub fn task2() {
    let actions: [i16; 6] = [1, 2, 1, 3, 4, 3];
    let chars: [char; 4] = ['A', 'B', 'C', 'D'];
    let sem = Arc::new(shared::sem::Sem::<4, i16>::new());
    let action_cnt = Arc::new(shared::sem::Sem::<6, i16>::new());
    let iteration_cnt = Arc::new(shared::sem::Sem::<6, i16>::new());
    let mut handles = vec![];

    for i in 0..4 {
        let _sem = Arc::clone(&sem);
        let _action_cnt = Arc::clone(&action_cnt);
        let _iteration_cnt = Arc::clone(&iteration_cnt);

        handles.push(thread::spawn(move || 'main: loop {
            // Wait until the semaphores internal value is equal to the current
            // threads index.
            _sem.wait_turn(i + 1);

            // If we printed `ABACDC` 5 times, notifiy next thread and exit.
            if _iteration_cnt.status() == 6 {
                _sem.signal();
                break 'main;
            }

            // Print current threads character.
            print!("{}", chars[i as usize]);

            // If we printed all 6 characters (Counting starts at 0)
            // print new line and increase iteration count.
            if _action_cnt.status() == 6 {
                println!();
                _iteration_cnt.next();
            }

            // Increment action count, in case it reaches max count,
            // it circles back to 0.
            _action_cnt.next();

            _sem.set(actions[_action_cnt.as_idx()]);
        }));
    }

    for t in handles {
        t.join().unwrap();
    }
}
