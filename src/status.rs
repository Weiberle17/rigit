use crate::repos::Repos;

// TODO: implement proper printing of status
// TODO: allow vor verbose flag
pub fn run_status(repos: Repos) {
  let status = get_status(repos);
}

// TODO: implement get_status function
pub fn get_status(repos: Repos) -> Result<String, String> {
  let mut results = Vec::new();
}
