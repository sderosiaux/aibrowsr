use crate::cdp::client::CdpClient;
use crate::snapshot::Snapshot;

pub async fn run(
    client: &CdpClient,
    verbose: bool,
    max_depth: Option<usize>,
    focus_uid: Option<&str>,
) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let snapshot = crate::snapshot::take_snapshot(client, verbose, max_depth, focus_uid).await?;
    Ok(snapshot)
}
