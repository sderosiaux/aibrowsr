use serde::Serialize;
use serde_json::json;

use crate::cdp::client::CdpClient;
use crate::cdp::types::GetTargetsResult;

/// Structured tab info for JSON output.
#[derive(Debug, Serialize)]
pub struct TabInfo {
    pub id: String,
    pub url: String,
    pub title: String,
}

/// Return structured tab data.
pub async fn run_structured(client: &CdpClient) -> Result<Vec<TabInfo>, Box<dyn std::error::Error>> {
    let result: GetTargetsResult = client
        .call("Target.getTargets", json!({}))
        .await?;

    let tabs = result
        .target_infos
        .into_iter()
        .filter(|t| t.target_type == "page")
        .map(|t| TabInfo {
            id: t.target_id,
            url: t.url,
            title: t.title,
        })
        .collect();

    Ok(tabs)
}

/// Return formatted text output (original behavior).
pub async fn run(client: &CdpClient) -> Result<String, Box<dyn std::error::Error>> {
    let tabs = run_structured(client).await?;

    if tabs.is_empty() {
        return Ok("No open tabs.".into());
    }

    let mut output = String::new();
    output.push_str(&format!(
        "{:<36}  {:<50}  {}\n",
        "TARGET_ID", "URL", "TITLE"
    ));
    output.push_str(&"-".repeat(120));
    output.push('\n');

    for tab in &tabs {
        let url_display = if tab.url.chars().count() > 50 {
            let truncated: String = tab.url.chars().take(47).collect();
            format!("{truncated}...")
        } else {
            tab.url.clone()
        };
        output.push_str(&format!(
            "{:<36}  {:<50}  {}\n",
            tab.id, url_display, tab.title
        ));
    }

    Ok(output)
}
