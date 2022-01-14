use crate::cmd::gh;
use octocrab::models::Project;
use std::process::Output;

pub fn list_prj() -> bool {
    let result = gh(&["api", "/repos/{owner}/{repo}/projects"]);
    let projects = extract_projects(result);
    if projects.is_empty() {
        // TODO: display owner/repo
        println!("This repository does not found any projects");
        return false;
    }

    for prj in projects {
        println!("{:?} {:?}", prj.number, prj.name);
    }

    true
}

fn extract_projects(result: Output) -> Vec<Project> {
    let stdout = std::str::from_utf8(&result.stdout).expect("Failed to covert str from API result");
    let projects =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");
    projects
}
