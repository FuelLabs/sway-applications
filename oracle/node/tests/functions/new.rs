mod oracle_node;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_initialize_oracle_node() {
        let oracle_node = oracle_node::OracleNode::new(10);
    }
}