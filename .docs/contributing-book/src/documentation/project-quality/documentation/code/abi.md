# ABI Documentation

ABI documentation refers to documenting the interface that another developer may be interested in using. 

The form of documentation we focus on uses the `///` syntax as we are interested in documenting the `ABI` functions.

In the following snippet, we provide a short description about the functions, the arguments they take, and when the calls will revert. Additional data may be added such as the structure of the return type, how to call the function, etc.

```sway
{{#include ../../../../code/connect_four/src/interface.sw:interface}}
```

<br>

In order to know what should be documented, the author of the code should put themselves in the position of a developer that knows nothing about the function and think about what sort of questions they may have.
