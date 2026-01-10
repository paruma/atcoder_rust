//! cargo-snippet 用の Linter
//!
//! #[snippet] 属性がついたモジュール内で `use crate::...` を使っていないかチェックします。
//! snippet 展開後は `crate` が存在しない（提出ファイルが crate になる）ため、
//! 相対パス（super:: や自身への参照）を使う必要があります。

use anyhow::Result;
use clap::Parser;
use colored::*;
use proc_macro2::TokenTree;
use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};
use syn::{ItemMod, ItemUse, UseTree};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target directory to check
    #[arg(default_value = "src/mylib")]
    dir: String,
}

struct SnippetLinter {
    current_file: PathBuf,
    in_snippet: bool,
    errors: Vec<String>,
}

impl SnippetLinter {
    fn new() -> Self {
        Self {
            current_file: PathBuf::new(),
            in_snippet: false,
            errors: Vec::new(),
        }
    }

    fn check_file(&mut self, path: &Path) -> Result<()> {
        self.current_file = path.to_path_buf();
        let content = std::fs::read_to_string(path)?;
        self.check_content(&content)
    }

    fn check_content(&mut self, content: &str) -> Result<()> {
        let file = syn::parse_file(content)?;
        self.visit_file(&file);
        Ok(())
    }
}

impl<'ast> Visit<'ast> for SnippetLinter {
    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        let has_snippet = i.attrs.iter().any(|attr| attr.path().is_ident("snippet"));
        let is_test_mod =
            i.ident == "tests" || i.ident == "test" || i.ident.to_string().starts_with("test_");

        let has_cfg_test = i.attrs.iter().any(|attr| {
            if attr.path().is_ident("cfg") {
                // Parse cfg(...) content
                if let Ok(syn::Meta::Path(path)) = attr.parse_args::<syn::Meta>() {
                    // Check for cfg(test)
                    return path.is_ident("test");
                }
                // cfg(test) often appears as a single path in the attribute parser logic depending on how it's invoked,
                // but strictly it might be `Meta::List` containing `test`.
                // Let's rely on token stream string matching for simplicity and robustness against parser versions.
                let tokens = attr
                    .meta
                    .require_list()
                    .map(|l| l.tokens.clone())
                    .unwrap_or_default();
                tokens.into_iter().any(|t| {
                    if let TokenTree::Ident(ident) = t {
                        ident == "test"
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        });

        let old_in_snippet = self.in_snippet;

        if has_snippet {
            self.in_snippet = true;
        }

        // If we enter a test module (unless it explicitly has #[snippet], which is rare/weird),
        // we disable the check.
        if (is_test_mod || has_cfg_test) && !has_snippet {
            self.in_snippet = false;
        }

        visit::visit_item_mod(self, i);

        self.in_snippet = old_in_snippet;
    }

    fn visit_item_use(&mut self, i: &'ast ItemUse) {
        if self.in_snippet {
            check_use_tree(&i.tree, &self.current_file, &mut self.errors);
        }
    }
}

fn check_use_tree(tree: &UseTree, file_path: &Path, errors: &mut Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            if path.ident == "crate" {
                // Found violation
                errors.push(format!(
                    "{}: '{}' is used inside a #[snippet] module.",
                    file_path.display().to_string().bold(),
                    "use crate::...".red()
                ));
            } else {
                // Recursively check if it's not crate::... (e.g. use std::collections::HashMap)
                check_use_tree(&path.tree, file_path, errors);
            }
        }
        UseTree::Name(_) => {}
        UseTree::Rename(_) => {}
        UseTree::Glob(_) => {}
        UseTree::Group(group) => {
            for item in &group.items {
                check_use_tree(item, file_path, errors);
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut linter = SnippetLinter::new();
    let mut has_error = false;

    for entry in WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let path = entry.path();
        if let Err(e) = linter.check_file(path) {
            eprintln!("Failed to parse {}: {}", path.display(), e);
        }
    }

    if !linter.errors.is_empty() {
        has_error = true;
        for err in linter.errors {
            eprintln!("{}", err);
        }
    }

    if has_error {
        std::process::exit(1);
    } else {
        println!("{}", "No snippet violation found.".green());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_snippet() {
        let code = r#"
            #[snippet]
            pub mod my_mod {
                use super::Other;
                use std::collections::HashMap;
            }
        "#;
        let mut linter = SnippetLinter::new();
        linter.check_content(code).unwrap();
        assert!(linter.errors.is_empty());
    }

    #[test]
    fn test_invalid_snippet_use_crate() {
        let code = r#"
            #[snippet]
            pub mod my_mod {
                use crate::some_mod::SomeStruct;
            }
        "#;
        let mut linter = SnippetLinter::new();
        linter.check_content(code).unwrap();
        assert!(!linter.errors.is_empty());
    }

    #[test]
    fn test_valid_in_tests() {
        let code = r#"
            #[snippet]
            pub mod my_mod {
                #[cfg(test)]
                mod tests {
                    use crate::some_mod::SomeStruct;
                }
            }
        "#;
        let mut linter = SnippetLinter::new();
        linter.check_content(code).unwrap();
        assert!(linter.errors.is_empty());
    }

    #[test]
    fn test_invalid_nested_snippet() {
        let code = r#"
            #[snippet]
            pub mod my_mod {
                pub mod inner {
                    use crate::some_mod::SomeStruct;
                }
            }
        "#;
        let mut linter = SnippetLinter::new();
        linter.check_content(code).unwrap();
        assert!(!linter.errors.is_empty());
    }

    #[test]
    fn test_no_snippet_attribute() {
        let code = r#"
            pub mod my_mod {
                use crate::some_mod::SomeStruct;
            }
        "#;
        let mut linter = SnippetLinter::new();
        linter.check_content(code).unwrap();
        assert!(linter.errors.is_empty());
    }
}
