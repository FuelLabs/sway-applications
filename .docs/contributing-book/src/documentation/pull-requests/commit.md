# Committing your work

Before a pull request can be made there must be work that has been committed and pushed. The content and descriptive message for the commit is important because it makes it easier to track the changes between each commit. Commits should generally be small and specific rather than megalithic changes that touch multiple parts of the codebase that conceptually have nothing to do with each other. The reason is that a small amount of work done is easier to reason about, alter and revert than 1 massive change across the entire codebase.

For example, let's say there are two changes that need to be made:

- A deposit function has a flaw in its assertion
- A separate contract needs additional data structures added in order to log events in a formalized structure (in a struct rather than emitting multiple logs, one per variable)

The correct approach would be to fix the deposit function in 1 commit and then add the formalized logging in a separate commit.

The commit message should be a short sentence describing the changes

- `Good commit message:` Fixed a conditional check in the deposit function which did not fail under condition XYZ
- `Bad commit message:`
  - Fix
  - Fixed function
  - Fixed assertion

It should be clear that the "bad" commit messages do not tell you what the change to the code is because they are too vague. Likewise, the "good" message _could_ be argued to be too long and thus it _could_ be deemed as a bad commit message by some - in this case the commit is acceptable because it addresses a single change nevertheless it's a good idea to refine your commit messages so that they are not too verbose / long. If you cannot cut down on the content of the commit message then that can be an indicator that too much work was done in 1 commit. That's not to say that you cannot write a paragraph in a commit however you need to know when that is applicable. For most use cases you shouldn't encounter problems if you keep your work small with a short descriptive sentence like the one above.

If you are unsure of what to write in a commit message then take a look at:

- The README from [joelparkerhenderson](https://github.com/joelparkerhenderson/git-commit-message/#git-commit-message)
- The [Medium article](https://medium.com/swlh/writing-better-commit-messages-9b0b6ff60c67) by Apurva Jain
