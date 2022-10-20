
#[tokio::main]
async fn main() {
    let oracle_node = node::OracleNode::new(10);
    oracle_node.run().await;
}
