# Creating a pull request

Creating a pull request is the last step if everything preceding it has been done correctly. 

In the description you should list

- The changes that have been made
- Limitations
- Assumptions 
- Future work if this PR is part of a set of PRs
- Additional notes (if relevant)
 
The information in your pull request should be structured neatly through the use of headings, bullet points, screenshots etc. This makes it easier to immediately see the changes rather than having to parse through 1 big paragraph. 

Lastly, you should reference the issue which spawned your pull request. 

You can 

- Reference it in a way that will not close the issue when the pull request is merged
  - Using a hyperlink 
  - GitHub syntax which is a hash symbol followed by the issue number e.g. #123
- Reference it in a way that will automatically close the original issue when the pull request is merged
  - GitHub has some special keywords referenced [here](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) which should be used in addition to the #123 syntax

The reason is that this allows the reviewer to quickly find the issue and check to see what work needed to be done and compare that against the pull request.

If your work has been accepted and you have an option to "squash" your commits then delete all the commit messages and summarize your work into a single commit message.
