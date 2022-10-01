# Style Guide

Programming languages have different ways of styling code i.e. how variables, functions, structures etc. are written.

The following snippets present the style for writing `Sway`.

## CapitalCase

Structs, traits, and enums are `CapitalCase` which means each word has a capitalized first letter. The fields inside a struct should be [snake_case](#snake_case) and `CapitalCase` inside an enum.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:structures}}
```

## snake_case

Modules, variables, and functions are `snake_case` which means that each word is lowercase and separated by an underscore.

Module name:

```sway
{{#include ../../../../code/style_guide/src/lib.sw:module}}
```

Function and variable:

```sway
{{#include ../../../../code/style_guide/src/lib.sw:function_case}}
```

## SCREAMING_SNAKE_CASE

Constants are `SCREAMING_SNAKE_CASE` which means that each word in capitalized and separated by an underscore.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:const}}
```

## Type Annotations

When declaring a variable it is possible to annotate it with a type however the compiler can usually infer that information.

The general approach is to omit a type if the compiler does not throw an error however if it is deemed clearer by the developer to indicate the type then that is also encouraged.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:type_annotation}}
```

## Field Initialization Shorthand

A struct has a shorthand notation for initializing its fields. The shorthand works by passing a variable into a struct with the exact same name and type.

The following struct has a field `amount` with type `u64`.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:struct_shorthand_definition}}
```

Using the shorthand notation we can initialize the struct in the following way.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:struct_shorthand_use}}
```

The shorthand is encouraged because it is a cleaner alternative to the following.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:struct_shorthand_avoid}}
```

## Getters

Getters should not follow the pattern of `get_XYZ()` and instead should follow `XYZ()`.

```sway
{{#include ../../../../code/style_guide/src/lib.sw:getters}}
```
