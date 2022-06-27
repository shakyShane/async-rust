mod channeltimer;

use futures::channel::mpsc;
use futures::executor::block_on;
use futures::StreamExt;

fn main() {
    block_on(async {
        let (_sender, receiver) = mpsc::channel::<()>(1);
        receiver.collect::<Vec<_>>().await
    });
}
