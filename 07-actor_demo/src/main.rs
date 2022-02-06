use actix::prelude::*;
use futures::future;
use futures::future::FutureExt;
use std::time::Duration;
use tokio::time;

struct Add(u32, u32);

impl Message for Add {
    type Result = Result<u32, ()>;
}

struct Adder;

impl Actor for Adder {
    type Context = SyncContext<Self>;
}

impl Handler<Add> for Adder {
    type Result = Result<u32, ()>;

    fn handle(&mut self, msg: Add, _: &mut Self::Context) -> Self::Result {
        let sum = msg.0 + msg.1;
        println!("Computed: {} + {} = {}", msg.0, msg.1, sum);
        Ok(msg.0 + msg.1)
    }
}

fn main() {
    let _ = actix::run(future::lazy(|_| {
        let addr = SyncArbiter::start(3, || Adder);
        for n in 5..10 {
            addr.do_send(Add(n, n + 1));
        }

        tokio::spawn(future::lazy(|_| {
            time::sleep(Duration::from_secs(1)).then(|_| {
                System::current().stop();
                future::ok::<(), ()>(())
            })
        }));
    }));
}
