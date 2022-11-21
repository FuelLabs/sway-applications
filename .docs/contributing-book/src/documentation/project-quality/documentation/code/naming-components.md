# Naming Components

Documenting the interface and adding comments is important however the holy grail is writing code that is self-documenting. 

Self-documenting code refers to code that written in such a way that a regular user who has never seen a line of code before could interpret what it is doing.

One of the most difficult aspects of programming is coming up with meaningful names that describe the content without being overly verbose while also not being too concise.

Naming components is both a skill and an art and there are many aspects to consider such as the context in which that variable may exist. In one context an abbreviated variable may be meaningful because of how fundamental that concept is while in another context it may be regarded as random characters.

Here are some points to consider when coming up with a name for a component.

## Abbreviations

Abbreviating names is a bad practice because it relies on contextual knowledge of the subject. It forces the developer to step away from their task in order to find the definition of some abbreviation or perhaps wait on a response from another developer.

On the other hand, common abbreviations may be meaningful for a given context and it may be detrimental to come up with a different, or long form, name to describe the content.

In general, a developer should take a moment to consider if an abbreviation provides more benefit than cost and how other developers may interpret that name in the given context.

That being said, here are some examples that should be avoided.

### Single Character Names

Using a single character to name a variable conveys little to no information to a developer.

- Is this a throw away variable?
- What is the variable meant to represent where ever it is used?
- Does it make sense to call it by the chosen character e.g. `x` when referring to forumlas?

### Ambigious Abbreviations

A common mistake is to abbreviate a variable when it does not need to be abbreviated or when the abbreviation may be ambigious.

For example, in the context of an industry that deals with temperature sensors what does the variable `temp` refer to?

- `temperature`
- `temporary`
- `tempo`

Perhaps in the specific function it makes sense to use the abbreviation. Nevertheless, it's better to add a few more characters to finish the variable name.

## Declarative statements

When choosing a name, the name should be a statement from the developer and not a question. Statements provide a simple true or false dynamic while a variable that may be read as a question provides doubt to the intended functionality.

For example:

- `can_change` -> `authorized`
  - The "can" can be read as a question or a statement. 
  - Is the developer asking the reader whether something can change or are they asserting that something either is or is not authorized to change?
- `is_on` -> `enabled`
  - "is" can also be read as a question posed to the reader rather than a simple declaration.
