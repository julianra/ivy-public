use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(about = "Generate a safe, offline HTML preview for an Ivy widget fixture")]
struct Args {
    widget: PathBuf,
    #[arg(short, long, default_value = "widget-preview.html")]
    output: PathBuf,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Widget {
    id: String,
    title: String,
    value: String,
    #[serde(default)]
    detail: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let widget: Widget = serde_json::from_str(
        &fs::read_to_string(&args.widget)
            .with_context(|| format!("could not read {}", args.widget.display()))?,
    )?;
    let html = format!(
        "<!doctype html><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>{title}</title><style>body{{margin:0;background:#101715;color:#edf7f1;font:16px system-ui;display:grid;place-items:center;min-height:100vh}}article{{width:min(28rem,80vw);background:#18231f;border:1px solid #355047;border-radius:1.2rem;padding:2rem;box-shadow:0 1rem 3rem #0008}}small{{color:#91b6a8}}h1{{font-size:1.2rem}}strong{{display:block;font-size:3rem;margin:.6rem 0}}</style><article data-widget-id=\"{id}\"><small>Ivy development widget</small><h1>{title}</h1><strong>{value}</strong><p>{detail}</p></article>",
        id = escape(&widget.id),
        title = escape(&widget.title),
        value = escape(&widget.value),
        detail = escape(&widget.detail),
    );
    fs::write(&args.output, html)?;
    println!("wrote {}", args.output.display());
    Ok(())
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::escape;

    #[test]
    fn html_is_escaped() {
        assert_eq!(escape("<script>&\""), "&lt;script&gt;&amp;&quot;");
    }
}
