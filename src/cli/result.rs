pub enum CliResult {
    ShowUI(crate::core::project::Project),
    Success,
    SaveChanges,
}
