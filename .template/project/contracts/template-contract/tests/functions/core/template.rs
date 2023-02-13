use crate::utils::{interface::core::template, setup::setup};

mod success {
    use super::*;

    #[tokio::test]
    async fn pass() {
        let (instance, _wallet) = setup().await;
        let _response = template(&instance).await;
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic]
    // #[should_panic(expected = "SomeErrorMessage")]
    async fn fail() {
        let (_instance, _wallet) = setup().await;
        assert!(false);
    }
}
