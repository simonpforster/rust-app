use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

async fn loadtest_list_users(user: &mut GooseUser) -> TransactionResult {
    let goose_users = user.get("/").await?;
    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_users, &validate).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Basic endpoint")
                .set_weight(10)?
                .register_transaction(transaction!(loadtest_list_users)),
        )
        .set_default(GooseDefault::Host, "http://localhost:8080")?
        .set_default(GooseDefault::NoPrintMetrics, true)?
        .set_default(GooseDefault::StartupTime, 30)?
        .set_default(GooseDefault::ThrottleRequests, 1000)?
        .execute()
        .await?;

    Ok(())
}
