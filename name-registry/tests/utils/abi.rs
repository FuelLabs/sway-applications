use crate::utils::*;

pub async fn extend(instance: &MyContract, name: &String, duration: u64) {
    instance
        .extend(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
        )
        .call_params(CallParameters {
            amount: duration / 100,
            asset_id: AssetId::BASE,
            gas_forwarded: Some(1000000),
        })
        .call()
        .await
        .unwrap();
}

pub async fn expiry(instance: &MyContract, name: &String) -> u64 {
    instance
        .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn identity(instance: &MyContract, name: &String) -> Identity {
    instance
        .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn owner(instance: &MyContract, name: &String) -> Identity {
    instance
        .owner(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn register(instance: &MyContract, name: &String, duration: u64, owner: &Identity, identity: &Identity) {
    instance
        .register(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
            owner.to_owned(),
            identity.to_owned()
        )
        .call_params(CallParameters {
            amount: duration / 100,
            asset_id: AssetId::BASE,
            gas_forwarded: Some(1000000),
        })
        .call()
        .await
        .unwrap();
}

pub async fn set_identity(instance: &MyContract, name: &String, identity: Identity) {
    instance
        .set_identity(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            identity,
        )
        .call()
        .await
        .unwrap();
}

pub async fn set_owner(instance: &MyContract, name: &String, new_owner: Identity) {
    instance
        .set_owner(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            new_owner,
        )
        .call()
        .await
        .unwrap();
}
