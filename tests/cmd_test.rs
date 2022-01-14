extern crate gh_prj;

#[test]
fn it_works_list_cmd() {
    assert!(gh_prj::cmd::list::list_prj());
}
