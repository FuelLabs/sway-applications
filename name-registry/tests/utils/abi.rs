use crate::utils::*;

pub async fn extend(instance: &NameRegistry, name: &String, duration: u64) {
    instance
    .methods()
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

pub async fn expiry(instance: &NameRegistry, name: &String) -> u64 {
    instance
    .methods()
        .expiry(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn identity(instance: &NameRegistry, name: &String) -> Identity {
    instance
    .methods()
        .identity(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn owner(instance: &NameRegistry, name: &String) -> Identity {
    instance.methods()
        .owner(SizedAsciiString::<8>::new(name.to_owned()).unwrap())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn register(
    instance: &NameRegistry,
    name: &String,
    duration: u64,
    owner: &Identity,
    identity: &Identity,
) {
    instance.methods()
        .register(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            duration,
            owner.to_owned(),
            identity.to_owned(),
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

pub async fn set_identity(instance: &NameRegistry, name: &String, identity: Identity) {
    instance.methods()
        .set_identity(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            identity,
        )
        .call()
        .await
        .unwrap();
}

pub async fn set_owner(instance: &NameRegistry, name: &String, new_owner: Identity) {
    instance.methods()
        .set_owner(
            SizedAsciiString::<8>::new(name.to_owned()).unwrap(),
            new_owner,
        )
        .call()
        .await
        .unwrap();
}
