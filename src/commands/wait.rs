use std::time::{Duration, Instant};

use serde_json::json;

use crate::cdp::client::CdpClient;
use crate::cdp::types::EvaluateResult;

/// Poll the page until a condition is met, or timeout.
pub async fn run(
    client: &CdpClient,
    what: &str,
    pattern: &str,
    timeout_secs: u64,
) -> Result<String, crate::BoxError> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let poll_interval = Duration::from_millis(200);

    let expression = match what {
        // Support both plain substrings and regex patterns (e.g. "Foo|Bar", "^Loading").
        // new RegExp(pattern) is backward-compatible with plain strings since every
        // literal string is a valid regex that matches itself.
        "text" => format!(
            "new RegExp({}).test(document.body.innerText)",
            serde_json::to_string(pattern)?
        ),
        "url" => format!(
            "location.href.includes({})",
            serde_json::to_string(pattern)?
        ),
        "selector" => format!(
            "!!document.querySelector({})",
            serde_json::to_string(pattern)?
        ),
        other => return Err(format!("Unknown wait type: {other}. Use \"text\", \"url\", or \"selector\".").into()),
    };

    loop {
        let result: EvaluateResult = client
            .call(
                "Runtime.evaluate",
                json!({
                    "expression": expression,
                    "returnByValue": true,
                }),
            )
            .await?;

        let matched = result
            .result
            .value
            .as_ref()
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        if matched {
            return Ok(format!("Found: {what} matching \"{pattern}\""));
        }

        if Instant::now() >= deadline {
            return Err(format!(
                "Timeout after {timeout_secs}s waiting for {what} matching \"{pattern}\""
            )
            .into());
        }

        tokio::time::sleep(poll_interval).await;
    }
}
