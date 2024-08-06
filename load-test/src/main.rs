mod scenarios;

use goose::prelude::*;
use scenarios::regular::regular;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Basic endpoint")
                .set_weight(10)?
                .register_transaction(transaction!(regular)),
        )
        .set_default(GooseDefault::ReportFile, "./target/loadtest_report.html")?
        .set_default(GooseDefault::Host, "http://localhost:8080")?
        .set_default(GooseDefault::NoPrintMetrics, false)?
        .set_default(GooseDefault::StartupTime, 30)?
        .set_default(GooseDefault::RunTime, 300)?
        .set_default(GooseDefault::ThrottleRequests, 300)?
        .execute()
        .await?;

    Ok(())
}
