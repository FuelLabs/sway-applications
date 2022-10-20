mod success {
    #[tokio::test]
    async fn can_get_price() {
        let oracle_node = node::OracleNode::new(10);
        let price = oracle_node.get_price().await;
        assert!(price > 0);
    }
}