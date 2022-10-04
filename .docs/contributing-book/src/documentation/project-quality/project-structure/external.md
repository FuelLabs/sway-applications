# External Project

If part of a project needs to be accessed externally, e.g., the ABI of a contract., the structure can be divided. 

Here is an example structure for an application that has a contract project and a library project. The contract depends on the library, which can be depended externally as well. 

```
application/
├── contract/
└── library/
```

The `contract` folder has a structure similar to an [internal project](internal.md), minus the `interface` part.

```
contract/
├── src/
├──── data_structures.sw
├──── errors.sw
├──── events.sw
├──── main.sw
├──── utils.sw
├── tests/
├── Cargo.toml
└── Forc.toml
```

The interface is in its own `library` project.

```
library/
├── src/
├──── lib.sw
├── tests/
├── Cargo.toml
└── Forc.toml
```
