extern crate futures;
extern crate tokio_core;

use std::thread;
use std::time::Duration;

use futures::{Future, Stream};
use futures::future::{Executor, ExecuteError, empty};
use futures::sync::mpsc;
use futures::sync::oneshot::{self, Execute};
use tokio_core::reactor::{Core, Remote};

struct MyExecutor {
    handle: Remote,
}

impl MyExecutor {
    fn new(handle: Remote) -> Self {
        MyExecutor { handle: handle }
    }
}

impl<F> Executor<Execute<F>> for MyExecutor
where
    F: Future + Send + 'static,
    F::Item: Send,
    F::Error: Send,
{
    fn execute(&self, future: Execute<F>) -> Result<(), ExecuteError<Execute<F>>> {
        self.handle.spawn(move |_| future);
        Ok(())
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let my_executor = MyExecutor::new(core.remote());
    thread::spawn(move || {
        let (tx, rx) = mpsc::unbounded();
        let f = rx.for_each(|t| Ok(println!("{}", t)));
        let handle = oneshot::spawn(f, &my_executor);
        handle.forget();
        for i in 1..10 {
            tx.unbounded_send(i).unwrap(); // panic here
            thread::sleep(Duration::from_secs(1));
        }
    });
    core.run(empty::<(), ()>()).unwrap();
}