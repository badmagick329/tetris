use tetris::run;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    run().await;
}
