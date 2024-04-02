use anyhow::anyhow;
use color_print::cprintln;
use std::io::IsTerminal;
use std::path::Path;
use std::{env, fs, io};

use termtree::Tree;

fn color() -> bool {
    env::var("NO_COLOR").is_err() && io::stdout().is_terminal()
}

pub fn print_tree(base: &Path) -> anyhow::Result<()> {
    let tree = tree(base)?;
    if color() {
        cprintln!("\n<yellow,bold>Password Store:</>");
    } else {
        println!("\nPassword Store:")
    }
    tree.to_string()
        .lines()
        .skip(1)
        .for_each(|s| println!("  {s}"));
    Ok(())
}

fn label<P: AsRef<Path>>(p: P) -> anyhow::Result<String> {
    p.as_ref()
        .file_name()
        .and_then(|o| o.to_str())
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow!("Could not read {:?}", p.as_ref()))
}

fn tree<P: AsRef<Path>>(p: P) -> anyhow::Result<Tree<String>> {
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
