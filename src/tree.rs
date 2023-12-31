use anyhow::anyhow;
use color_print::cprintln;
use std::fs;
use std::path::Path;

use termtree::Tree;

use crate::error::ParResult;

pub fn print_tree(base: &Path) -> ParResult<()> {
    let tree = tree(base)?;
    cprintln!("\n<yellow,bold>Password Store:</>");
    tree.to_string()
        .lines()
        .skip(1)
        .for_each(|s| println!("  {s}"));
    Ok(())
}

fn label<P: AsRef<Path>>(p: P) -> ParResult<String> {
    p.as_ref()
        .file_name()
        .and_then(|o| o.to_str())
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow!("Could not read {:?}", p.as_ref()))
}

fn tree<P: AsRef<Path>>(p: P) -> ParResult<Tree<String>> {
    fs::read_dir(&p)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|o| o.to_str())
                .map(|s| !s.starts_with('.'))
                .unwrap_or(true)
        })
        .filter_map(|e| label(e.path()).map(|l| (e, l)).ok())
        .try_fold(
            Tree::new(label(p.as_ref().canonicalize()?)?),
            |mut root, (entry, label)| {
                let dir = entry.metadata()?;
                if dir.is_dir() {
                    root.push(tree(entry.path())?);
                } else {
                    root.push(Tree::new(label));
                }
                Ok(root)
            },
        )
}
