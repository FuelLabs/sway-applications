# Creating a pull request

There are two types of pull requests and depending on which one is chosen it will convey a different intent to the authors.

A `regular` pull request is for when the author of the pull request is satisfied with the work done and believes that the author(s) of the project should perform a review in preparation of merging the work into some branch.

A `draft` pull request indicates that work is currently in progress and not ready for review.

## When to create a pull request

There are two approaches that can be taken:

- A pull request can be made when the task is deemed to be completed
- A `draft` pull request can be created after the first commit in order to allow for easy tracking of the progress

Which one should be chosen may come down to preference or the contributing guide of a project. That being said, the benefit of creating and working on a `draft` is that it makes it easier to spot the request and thus early comments may be left which provide additional support.

## How to structure a pull request

Depending on the account permissions and where the pull request is being made, there may be some features that are unavailable. For example, an external contributor may not be able to set a label to categorize the request.

There are at least five sections to consider when creating a pull request:

<!-- no toc --> 
- [The Title](#the-title)
- [The Description](#the-description)
- [The Reviewers](#the-reviewers)
- [The Labels](#the-labels)
- [Linked Issues](#the-issues)
- [Merging the Pull Request](#merging-the-pull-request)

### The Title

It's important to provide a title that accurately identifies the work that is being done. This is easy if there is an issue, even more so if the issue is described well, as the title can be directly copy and pasted from the issue. This allows for a one-to-one mapping of an issue to pull request which makes it easy to spot when an issue is ready to be merged.

### The Description

The information in the pull request should be structured neatly through the use of headings, bullet points, screenshots etc. because this makes it easier to immediately see the changes rather than having to parse through one large paragraph.

Some ideas for sections are:

- The changes that have been made and the motivation behind them
- Limitations
- Assumptions 
- Future work if the pull request is part of an epic (set of tasks / issues)

### The Reviewers

If the project is managed well then a contributor does not have to think about who should review their work because it will be automatically filled in for them. This is done through the use of a [code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) file.

If that is not the case then the contributor will need to figure out the correct author(s) for code review and select them (if permissions allow it) or the request will be without any reviews until an author spots the request and assigns someone.

### The Labels

If there is an issue which is well managed then the labels for that issue can be set on the pull request (if permissions allow it) otherwise an author may need to set the labels if they choose to.

### The Issues

If there is an issue that the pull request is working off of then it's a good practice to reference that issue so that it gets closed automatically when the pull request is merged. This can be done via the user interface or by reference in the description using a [closing keyword](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue).

For example, issue number `123` would be referenced in the description as `closes #123`.

Additionally, referencing the issue that the pull request is based on allows the reviewer to easily click on the link which will take them to the issue. This makes it easy to see the problem in detail and any discussion that occurred.

### Merging the Pull Request

Once the request has received enough approvals from the authors then either the authors or the contributor may merge the work in. When attempting to merge there may be an option to [squash](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/about-pull-request-merges#squash-and-merge-your-commits) the commits. It's a good idea to delete the previous commits in the optional description so that a single message summarizes the entire work that has been done. This makes it easier to parse the commit history.
