# Committing your work

A `commit` can be thought of as a snapshot in time which captures the difference between the snapshot that is currently being made and the previous snapshot.

When creating a snapshot there are some points to consider in order to keep a high quality log:

- The quantity of work between commits
- Grouping work by task / concept
- The message used to track the changes between commits

## Quantity of Work

The amount of work done per commit is dependent upon the task that is being solved however there is a general rule to follow and that is to avoid the extremes of committing everything at once or committing every minor change such as a typo.

The reason for not committing all of the work at once is twofold:

- When a fault occurs which leads to a loss of work then all of that work is lost
- If a section of work needs to be reverted then everything must be reverted

Similarly, small commits should be avoided because:

- A lot of commits may be considered as spam and may be difficult to parse

## Categorization

Categorizing commits into issues being resolved allows us to easily scope the amount of work per commit. With appropriate categories the likelihood of too much, or not enough, work being committed is reduced.

An example could be a failing test suite which includes multiple functions that were re-written. In this instance it may be a good idea to fix a test, or a test suite, for one specific function and committing that work before moving onto the next. 

This creates a clear separation within the task of fixing the test suites by fixing one suite in one commit and another in another commit.

## Commit Messages

Once the issue has been resolved it's time to write a message that will distinguish this commit from any other.

The commit message should be a concise and accurate summary of the work done:

- `Good commit message:` 
  - Fixed precondition in `withdraw()` which allowed draining to occur
- `Bad commit message:`
  - Fix
  - Fixed function
  - Fixed an assertion where a user is able to repeatedly call the `withdraw()` functions under an edge case which may lead to the contract being drained

More information about commit messages may be found in:

- The README from [joelparkerhenderson](https://github.com/joelparkerhenderson/git-commit-message/#git-commit-message)
- The [Medium article](https://medium.com/swlh/writing-better-commit-messages-9b0b6ff60c67) by Apurva Jain
