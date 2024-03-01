use goose::goose::{GooseUser, TransactionResult};
use goose_eggs::{validate_and_load_static_assets, Validate};

pub async fn regular(user: &mut GooseUser) -> TransactionResult {
    let goose_users = user.get("/").await?;
    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_users, &validate).await?;

    Ok(())
}
