use anyhow::Error;
use pulldown_cmark::HeadingLevel::H1;
use pulldown_cmark::{Event, Options, Parser, Tag};
#[allow(clippy::single_component_path_imports)]
use reqwest;
use std::collections::BTreeMap;
use std::fs::File;

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    let _support_versions = vec![
        "1.16", "1.17", "1.18", "1.19", "1.20", "1.21", "1.22", "1.23", "1.24", "1.25", "1.26",
        "1.27", "1.28", "1.29", "1.30", "1.31",
    ];
    let mut versions = BTreeMap::new();
    for version in _support_versions {
        let url = format!(
            "https://raw.githubusercontent.com/kubernetes/kubernetes/refs/heads/master/CHANGELOG/CHANGELOG-{}.md",
            version
        );
        println!("cargo:warning={}", url);
        let resp = reqwest::get(&url).await?;
        let body = resp.text().await?;
        let h1 = parse_md(&body)?;
        println!("cargo:warning=fetch version is {:?}", h1);
        versions.insert(version.to_string(), h1);
    }
    let file = File::create("files/kube_versions.json")?;
    serde_json::to_writer_pretty(file, &versions)?;
    Ok(())
}
#[allow(clippy::redundant_guards)]
fn parse_md(markdown_input: &str) -> Result<Vec<String>, Error> {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);
    let mut titles = Vec::new();
    let mut in_level_one_heading = false;
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) if level == H1 => {
                in_level_one_heading = true;
            }
            Event::Text(text) if in_level_one_heading => {
                in_level_one_heading = false;
                if text.contains('-') || !text.starts_with('v') {
                    continue;
                }
                titles.push(text.to_string());
            }
            _ => {}
        }
    }
    Ok(titles)
}
