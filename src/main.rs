use serde_derive::*;
use serde_json::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub guid: String,
    pub title: String,
    pub index: i64,
    pub date_added: i64,
    pub last_modified: i64,
    pub id: i64,
    pub type_code: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub root: Option<String>,
    pub children: Option<Vec<Root>>,
    pub uri: Option<String>,
}

fn main() -> Result<()> {
    let mut args = std::env::args();

    let filename = args.nth(1).unwrap();
    let data = std::fs::read_to_string(&filename).unwrap();

    let r: Root = serde_json::from_str(&data)?;

    let idx = 1;
    if let Some(_) = r.children {
        print_header();
        print_children(idx, &r);
    } else {
        eprintln!("JSON file {} has no children!", filename);
    }

    Ok(())
}

fn print_header() {
    let now_str = chrono::offset::Utc::now()
        .format("%Y-%m-%d")
        .to_string();

    println!(r##"---
title: Bookmarks
description: My Bookmarks
date: {}
tags:
- bookmarks
- links
categories:
- Bookmarks
---
An auto-generated list of my FireFox bookmarks.
"##, now_str);

}

fn print_children(idx: usize, r: &Root) {
    if let Some(c) = &r.children {
        for x in c {
            if x.type_code == 2 {
                let prefix = "#".repeat(idx);
                println!("{} {}", prefix, x.title);
                print_children(idx+1, &x)
            }
            else if x.type_code == 1 {
                let prefix = "*".repeat(1);
                if let Some(u) = &x.uri {
                    println!("{} [{}]({})", prefix, x.title, u);
                }
            }
        }
    }
}
