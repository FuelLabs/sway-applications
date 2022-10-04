# Project Structure

In order to navigate through a project easily, there needs to be a structure that compartmentalizes concepts. This means that code is grouped together based on some concept. Before laying out the project structure, let's talk about Sway project types.

Every Sway project has an entry point that is specified in its manifest file.

```
[project]
authors = ["Fuel Labs <contact@fuel.sh>"]
entry = "main.sw"
license = "Apache-2.0"
name = "manifest_example"
```

The type of the entry point is specified in its first line, just as in every Sway program. It can be one of the four Sway program types: a *contract*, a *predicate*, a *script* or a *library*. 

A *library project*, i.e., a project which has a *library* entry point, can be added as a dependency to an external project. 

A *contract project*, on the other hand, is not accessible from external projects since contracts cannot be added as dependencies. For this reason, other files in this *contract project* that are not `contract`s, but maybe are `library`s, may be abstracted away from external projects.

The following sections will outline the structures of broadly two kinds of projects:

- [Internal project](internal.md)
  - Projects where external access is natural or unnecessary
- [External project](external.md)
  - Projects where external access is made possible