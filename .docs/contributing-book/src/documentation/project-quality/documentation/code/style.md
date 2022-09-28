# Style Guide

Programming languages have different ways of styling code i.e. how variables, functions, structures etc. are written.

The following snippets present the style for writing `Sway`.

## CapitalCase

Structs, traits, and enums are `CapitalCase` which means each word has a capitalized first letter. The fields inside a struct should be [snake_case](#snake_case) and `CapitalCase` inside an enum.

```rust
{{#include ../../../../code/style_guide/src/lib.sw:structures}}
```

## snake_case

Modules, variables, and functions are `snake_case` which means that each word is lowercase and separated by an underscore.

Module name:

```rust
{{#include ../../../../code/style_guide/src/lib.sw:module}}
```

Function and variable:

```rust
{{#include ../../../../code/style_guide/src/lib.sw:function_case}}
```

## SCREAMING_SNAKE_CASE

Constants are `SCREAMING_SNAKE_CASE` which means that each word in capitalized and separated by an underscore.

```rust
{{#include ../../../../code/style_guide/src/lib.sw:const}}
```

## Type Annotations

When declaring a variable it is possible to annotate it with a type however the compiler can usually infer that information.

The general approach is to omit a type if the compiler does not throw an error however if it is deemed clearer by the developer to indicate the type then that is also encouraged.

```rust
{{#include ../../../../code/style_guide/src/lib.sw:type_annotation}}
```

## Getters

Getters should not follow the pattern of `get_XYZ()` and instead should follow `XYZ()`.

```rust
{{#include ../../../../code/style_guide/src/lib.sw:getters}}
```
