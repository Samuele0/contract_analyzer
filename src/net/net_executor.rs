use super::transaction::Transaction;
use scoped_threadpool::Pool;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
pub fn execute_net(start: Vec<Arc<Mutex<Transaction>>>, tot_transactions: usize) {
    let mut pool = Pool::new(4);
    let (sender, receiver) = channel::<Arc<Mutex<Transaction>>>();
    let start_tx = sender.clone();
    pool.scoped(|s| {
        s.execute(move || {
            for t in start {
                start_tx.send(t.clone()).unwrap();
            }
        });
        for _ in 0..tot_transactions {
            let tr = receiver.recv().unwrap();
            let thread_tx = sender.clone();
            s.execute(move || {
                tr.lock().unwrap().run();
                for t in tr.lock().unwrap().ended() {
                    thread_tx.send(t).unwrap();
                }
            });
        }
    });
}
