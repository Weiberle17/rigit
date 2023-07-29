# rigit
Tool which let's you perform a git actions on multiple repositories.

## Currently working commands:
- -h --help: display help message
- -V --version: displays the current release version
- status: displays the status of all repositories under the specified directory
  - allows -v --verbose flag to display complete git status
- fetch: pull the remote changes of all repositories under the specified directory but don't merge/rebase automatically.

## WIP
- pull: pull the remote changes of all repositories under the specified directory and merge/rebase them.

## Planned features
- push: push the local changes of all repositories under the specified directory to the remote repositories.
