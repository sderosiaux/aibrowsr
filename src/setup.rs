//! Page-level setup: console interceptor injection and stealth patches.
//!
//! Extracted from main.rs to keep it under the line limit.

use serde_json::json;

use crate::cdp::client::CdpClient;

/// Apply stealth anti-detection patches. Must be called after `Page.enable`.
pub async fn apply_stealth(client: &CdpClient) {
    let _ = client.enable("Network").await;

    // 1. navigator.webdriver = undefined + other fingerprint patches
    // Injected before ANY page JS runs, survives navigations
    let _ = client
        .send(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": STEALTH_PATCHES_JS }),
        )
        .await;

    // 2. Patch the current page immediately (in case we connected mid-session)
    let _ = client
        .send(
            "Runtime.evaluate",
            json!({"expression": "Object.defineProperty(navigator, 'webdriver', { get: () => undefined });"}),
        )
        .await;

    // 3. Override user-agent to remove "HeadlessChrome"
    let _ = client
        .send(
            "Network.setUserAgentOverride",
            json!({
                "userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
                "acceptLanguage": "en-US,en;q=0.9",
                "platform": "MacIntel"
            }),
        )
        .await;
}

// ---------------------------------------------------------------------------
// JS source constants
// ---------------------------------------------------------------------------

const STEALTH_PATCHES_JS: &str = r#"
    Object.defineProperty(navigator, 'webdriver', { get: () => undefined });
    // Mask chrome.runtime (headless doesn't have it)
    if (!window.chrome) window.chrome = {};
    if (!window.chrome.runtime) window.chrome.runtime = { connect: () => {}, sendMessage: () => {} };
    // Mask Permissions API inconsistency (headless returns "prompt" for notifications)
    const origQuery = window.Permissions && Permissions.prototype.query;
    if (origQuery) {
        Permissions.prototype.query = (params) => (
            params.name === 'notifications'
                ? Promise.resolve({ state: Notification.permission })
                : origQuery.call(Permissions.prototype, params)
        );
    }
    // Mask webGL vendor/renderer (headless gives "Google Inc." / "ANGLE")
    const getParam = WebGLRenderingContext.prototype.getParameter;
    WebGLRenderingContext.prototype.getParameter = function(param) {
        if (param === 37445) return 'Intel Inc.';
        if (param === 37446) return 'Intel Iris OpenGL Engine';
        return getParam.call(this, param);
    };
    // Fix CDP input leak: screenX/screenY == pageX/pageY reveals automation.
    const __screenOffset = { x: Math.floor(Math.random() * 100) + 50, y: Math.floor(Math.random() * 100) + 80 };
    const origMouseEvent = MouseEvent;
    window.MouseEvent = class extends origMouseEvent {
        constructor(type, init = {}) {
            if (init.screenX !== undefined) init.screenX += __screenOffset.x;
            if (init.screenY !== undefined) init.screenY += __screenOffset.y;
            super(type, init);
        }
    };
"#;
