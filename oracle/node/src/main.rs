mod oracle_node;

#[tokio::main]
async fn main() {
    let oracle_node = oracle_node::OracleNode::new(10);
    oracle_node.run().await;
}   
