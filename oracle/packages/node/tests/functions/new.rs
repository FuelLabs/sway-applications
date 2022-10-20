mod success {
    #[tokio::test]
    async fn can_initialize_oracle_node() {
        let oracle_node = node::OracleNode::new(10);
    }
}