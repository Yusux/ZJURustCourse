mod walk_tree;

use regex::RegexSet;
use std::{path::Path, collections::BTreeSet};
use tracing::{info, span};

pub fn find<P: AsRef<Path>>(
    root: &[P],
    regex: &RegexSet,
    is_verbose: &bool,
    is_color: &bool
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // set span for find
    let span = span!(tracing::Level::TRACE, "find");
    let _entry = span.enter();

    // use BTreeSet to store the matching files to avoid duplication
    let mut matches = BTreeSet::new();
    
    // walk the tree for each path in root
    info!("start walking tree");
    for path in root {
        info!("start walking tree: {}", path.as_ref().to_string_lossy());
        walk_tree::walk_tree(path.as_ref(), regex, &mut matches, is_verbose, is_color)?;
        info!("finish walking tree: {}", path.as_ref().to_string_lossy());
    }
    info!("finish walking tree");

    // turn BTreeSet into Vec
    Ok(matches.into_iter().collect())
}