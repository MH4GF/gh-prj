use crate::cmd::gh;
use octocrab::models::Project;

pub fn list_prj() -> bool {
    let result = gh(&["api", "/repos/{owner}/{repo}/projects"]);
    let stdout = match std::str::from_utf8(&result.stdout) {
        Err(e) => {
            eprintln!("Failed to covert str from API result: {}", e);
            return false;
        }
        Ok(str) => {
            if str.is_empty() {
                eprintln!("Failed to execute gh command: {:?}", &result.stderr);
                return false;
            }
            str
        }
    };
    let projects: Vec<Project> =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");

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
