use fantoccini::{wd::WebDriverCompatibleCommand, Client};
use reqwest::Method;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
struct ExecuteCdp(String, Value);

impl WebDriverCompatibleCommand for ExecuteCdp {
    fn endpoint(
        &self,
        base_url: &url::Url,
        session: Option<&str>,
    ) -> Result<url::Url, url::ParseError> {
        base_url.join(&format!(
            "session/{}/goog/cdp/execute",
            session.as_ref().unwrap()
        ))
    }

    fn method_and_body(&self, _: &url::Url) -> (reqwest::Method, Option<String>) {
        (
            Method::POST,
            Some(json!({"cmd": self.0, "params": self.1 }).to_string()),
        )
    }
}

async fn on_load(client: &Client, code: &str, args: Vec<Value>) {
    let expr = format!(
        "({})({})",
        code.trim().trim_end_matches(';'),
        args.into_iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("{expr}");
    client
        .issue_cmd(ExecuteCdp(
            "Page.addScriptToEvaluateOnNewDocument".to_owned(),
            json!({ "source": expr }),
        ))
        .await
        .expect("Failed issuing cmd");
}

pub(crate) async fn evade(client: &Client) {
    tokio::join!(
        on_load(client, include_str!("./evade/utils.js"), vec![]),
        on_load(client, include_str!("./evade/chrome.app.js"), vec![]),
        on_load(client, include_str!("./evade/chrome.csi.js"), vec![]),
        on_load(client, include_str!("./evade/chrome.loadTimes.js"), vec![]),
        on_load(
            client,
            include_str!("./evade/chrome.runtime.js"),
            vec![Value::Bool(false)]
        ),
        on_load(
            client,
            include_str!("./evade/iframe.contentWindow.js"),
            vec![]
        ),
        on_load(client, include_str!("./evade/media.codecs.js"), vec![]),
        on_load(
            client,
            include_str!("./evade/navigator.hardwareConcurrency.js"),
            vec![Value::Number(4.into())]
        ),
        on_load(
            client,
            include_str!("./evade/navigator.languages.js"),
            vec![Value::Array(vec!["en-US".into(), "en".into()])]
        ),
        on_load(
            client,
            include_str!("./evade/navigator.permissions.js"),
            vec![]
        ),
        on_load(client, include_str!("./evade/navigator.plugins.js"), vec![]),
        on_load(
            client,
            include_str!("./evade/navigator.vendor.js"),
            vec![Value::String("Google Inc.".to_owned())]
        ),
        on_load(
            client,
            include_str!("./evade/navigator.webdriver.js"),
            vec![json!("Google Inc.")]
        ),
        on_load(client, include_str!("./evade/webgl.vendor.js"), vec![]),
        on_load(
            client,
            include_str!("./evade/window.outerdimensions.js"),
            vec![]
        ),
    );
}
