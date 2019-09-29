extern crate regex;

use std::fs;
use regex::Regex;

fn ignored_branches(branch_name: &&str) -> bool {
    (!branch_name.contains("master"))
}

/**
 * Find branches that are tracking a remote
 */
pub fn retrieve() -> Vec<String> {
    let git_config = fs::read_to_string(".git/config")
        .unwrap();
    let re = Regex::new(r#"^\[branch "([^"]*)"]$"#).unwrap();
    
    let branch_names: Vec<String> = git_config.lines()
        .map(|line| {
            let trimmed_line: &str = line.trim();
            let branch_capture = re.captures(trimmed_line);
            match branch_capture {
                None => None,
                Some(captures) => Some(captures.get(1).unwrap().as_str())
            }
        })
        .filter_map(|line| line) // remove any None objects from the list and return tge Some value
        .filter(ignored_branches)
        .map(|line| line.to_string())
        .collect();

    (branch_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignored_branches_valid() {
        let test_str = "origin/master";
        assert!(!ignored_branches(&test_str));
    }

    #[test]
    fn ignored_branches_invalid() {
        let test_str = "origin/develop";
        assert!(ignored_branches(&test_str));
    }
}