pub fn get_version() -> String {
    crate::git::get_tag()
}