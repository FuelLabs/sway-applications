# Comments

Comments are used by developers for themselves or other developers to provide insight into some functionality.

There are many ways to achieve the same outcome for a line of code however there are implementation tradeoffs to consider and a developer might be interested in knowing why the current approach has been chosen. 

Moreover, it may not be immediately clear why, or what, some line of code is doing so it may be a good idea to add a comment summarizing the intent behind the implementation.

The following snippet looks at two items being documented using the comment syntax `//`.

- `Item1` has poor comments that do not convey any meaningful information and it's better to not include them at all.
- `Item2` has taken the approach of describing the context in order to provide meaning behind each field

```sway
{{#include ../../../../code/bad_documentation/src/lib.sw:data_structures}}
```
