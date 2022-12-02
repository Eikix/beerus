use beerus_core::{
    config::Config,
    lightclient::beerus::{Beerus, BeerusLightClient},
};
use eyre::Result;
use log::debug;

/// Query the StarkNet state root.
pub async fn query_starknet_state_root(config: Config) -> Result<()> {
    debug!("Querying the StarkNet state root...");
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(config)?;
    // Start the Beerus light client.
    debug!("Starting the Beerus light client...");
    beerus.start().await?;
    debug!("Beerus light client started and synced.");
    // Call the StarkNet contract to get the state root.
    let state_root = beerus.starknet_state_root().await?;
    println!("{}", state_root);
    Ok(())
}
