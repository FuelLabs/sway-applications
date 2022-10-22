use std::thread;

mod success {
    use std::thread::{self, sleep};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn can_run() {
        let oracle_node = node::OracleNode::new(10);
        oracle_node.run();

        while let Some(price) = oracle_node.receiver.recv().await {
            println!("got: {}", price);
        }
    }
}