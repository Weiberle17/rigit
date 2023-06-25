pub fn status() {
  
}
pub fn check_status<'a>(contents: &'a str) -> Result<Vec<&'a str>, String> {
  let mut results = Vec::new();

  if !contents.contains("up to date") {
    results.push("Local repository is not synchronized with the remote repository.");
  }
  if contents.contains("modified") {
    results.push("You have uncommited changes in your local repository.");
  }
  if contents.contains("untracked") || contents.contains("new file") {
    results.push("You have untracked files in your repository")
  }
  if results.is_empty() {
    return Err("The repository is clean!".to_string());
  }
  
  Ok(results)
}

#[cfg(test)]
mod tests {
  use super::*;
}
