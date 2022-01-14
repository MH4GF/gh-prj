use crate::cmd::{gh, open};
use octocrab::models::Project;

pub fn view_prj(web_mode: bool, arg: isize) -> bool {
    let query = format!(
        "limit(1; .[] | select(.number == {i}))",
        i = &arg.to_string()
    );
    let result = gh(&["api", "/repos/{owner}/{repo}/projects", "-q", &query]);
    let stdout = std::str::from_utf8(&result.stdout).expect("Failed to covert str from API result");
    if stdout.is_empty() {
        println!("Failed to execute gh command: {:?}", &result.stderr);
        return false;
    }

    let project: Project =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");

    if web_mode {
        let url = project.html_url;
        let url_str: String = url.into();
        open(&[&url_str]);
        println!("Opening {:?} in your browser.", &url_str);

        return true;
    }

    // TODO
    println!("{:?}", project);
    println!("Not implemented view command");

    return true;
}
