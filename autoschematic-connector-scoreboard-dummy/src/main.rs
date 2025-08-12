use autoschematic_core::{grpc_bridge::grpc_connector_main, tarpc_bridge::tarpc_connector_main};
use connector::ScoreboardConnector;

pub mod addr;
pub mod connector;
pub mod op;
pub mod resource;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let protocol = std::env::var("PROTOCOL");
    if protocol == Ok(String::from("GRPC")) {
        grpc_connector_main::<ScoreboardConnector>().await?;
    } else {
        tarpc_connector_main::<ScoreboardConnector>().await?;
    }
    Ok(())
}
