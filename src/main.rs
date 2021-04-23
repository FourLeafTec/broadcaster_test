use broadcaster::BroadcastChannel;
use futures;
use futures_util::StreamExt;
use tokio::spawn;
use tokio::time;

#[tokio::main()]
async fn main() {
    let chan = BroadcastChannel::<usize>::new();
    let mut handlers = vec![];
    for i in 0..10 {
        {
            let chan = chan.clone();
            handlers.push(spawn(async move {
                let mut chan = chan.clone();
                loop {
                    let _rd = chan.next().await;
                    println!("recv {}:{:?}", i, _rd);
                }
            }));
        }
    }

    {
        let chan = chan.clone();
        handlers.push(spawn(async move {
            time::sleep(std::time::Duration::from_secs(2)).await;

            for _i in 0..10 {
                let _chan = chan.clone();
                spawn(async move { _chan.send(&1).await.unwrap() });
            }
        }));
    }
    futures::future::join_all(handlers).await;
}
