use helpers::print_all_issues;
use regex::Regex;
use std::{collections::HashSet, env, fs};

mod constants;
mod helpers;
mod types;

use types::{Issue, IssueType};

const FILE_EXT: &str = ".rs";

// TODO: first todo
// TODO: second todo

fn find_todos(file_contents: &String) -> Vec<Issue> {
    let re = Regex::new("^(//|#|--) (TOD(O*)|FIXM(E*)):(.*)").unwrap();

    // Capture 0 - Entire match
    // Capture 1 - Comment symbol
    // Capture 2 - TODO | FIXME
    // Capture 3 - O | None
    // Capture 4 - E | None
    // Capture 5 - Description

    let mut vector: Vec<Issue> = Vec::new();

    for line in file_contents.split("\n") {
        if let Some(captures) = re.captures(&line) {
            if let Some(description) = captures.get(5) {
                // only add if description exists

                let issue_type = IssueType::from_str(&captures[2]);

                let priority = match issue_type {
                    IssueType::Todo => {
                        let string = &captures[3];
                        string.len()
                    }

                    IssueType::Fixme => {
                        let string = &captures[4];
                        string.len()
                    }
                };

                vector.push(Issue {
                    issue_type,
                    priority,
                    description: description.as_str().to_string(),
                });
            };
        }
    }

    vector
}

fn walk_dirs(path: &String, folders_to_ignore: &HashSet<&str>, all_issues: &mut Vec<Issue>) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let current_path = file.unwrap().path();

        let current_path_str = current_path.to_str().unwrap();

        // TODO: Refactor this thing
        if current_path.is_file()
            && (current_path_str.ends_with(FILE_EXT) || current_path_str.ends_with(".py"))
        {
            match fs::read_to_string(&current_path) {
                Ok(file_content) => {
                    all_issues.extend(find_todos(&file_content));
                }

                Err(error) => {
                    println!("Failed to read file {}. Error: {}", current_path_str, error);
                    continue;
                }
            }
        } else if current_path.is_dir() {
            // ignore symlinks
            let splits: Vec<&str> = current_path_str.split("/").collect();
            let dir_name = *splits.last().unwrap();

            if folders_to_ignore.contains(dir_name) {
                println!("{} in ignore list. Igonoring", dir_name);
                continue;
            }

            walk_dirs(
                &String::from(current_path_str),
                folders_to_ignore,
                all_issues,
            );
        }
    }
}

fn main() {
    let folders_to_ignore: HashSet<&str> =
        HashSet::from([".git", "node_modules", "target", "dist", "env"]);

    let args: Vec<String> = env::args().collect();

    let mut cwd = &String::from(env::current_dir().unwrap().to_str().unwrap());

    if args.len() > 1 {
        cwd = &args[1];
    }

    let mut all_issues: Vec<Issue> = Vec::new();

    walk_dirs(cwd, &folders_to_ignore, &mut all_issues);

    print_all_issues(&all_issues);
}
