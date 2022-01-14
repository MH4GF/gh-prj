extern crate gh_prj;

/// fetch data from https://github.com/MH4GF/gh-prj/projects
#[test]
fn it_works_list_cmd() {
    assert!(gh_prj::cmd::list::list_prj());
}

/// fetch data from https://github.com/MH4GF/gh-prj/projects/1
#[test]
fn it_works_view_cmd() {
    assert!(gh_prj::cmd::view::view_prj(false, 1));
}
