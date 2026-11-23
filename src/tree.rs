//! Render file paths as an indented tree.

use std::collections::BTreeMap;

#[derive(Default)]
struct Node {
    dirs: BTreeMap<String, Node>,
    files: Vec<String>,
}

/// Render sorted paths like `src/main.rs` as a tree with ├── and └──.
pub fn render(paths: &[String]) -> String {
    let mut root = Node::default();
    for path in paths {
        let mut node = &mut root;
        let parts: Vec<&str> = path.split('/').collect();
        for dir in &parts[..parts.len().saturating_sub(1)] {
            node = node.dirs.entry((*dir).to_string()).or_default();
        }
        if let Some(file) = parts.last() {
            node.files.push((*file).to_string());
        }
    }
    let mut out = String::new();
    write_node(&root, "", &mut out);
    out
}

fn write_node(node: &Node, prefix: &str, out: &mut String) {
    let entries = node.dirs.len() + node.files.len();
    let mut i = 0;
    for (name, child) in &node.dirs {
        i += 1;
        let last = i == entries;
        let (arm, pad) = if last { ("└── ", "    ") } else { ("├── ", "│   ") };
        out.push_str(&format!("{prefix}{arm}{name}/\n"));
        write_node(child, &format!("{prefix}{pad}"), out);
    }
    for name in &node.files {
        i += 1;
        let last = i == entries;
        let arm = if last { "└── " } else { "├── " };
        out.push_str(&format!("{prefix}{arm}{name}\n"));
    }
}
