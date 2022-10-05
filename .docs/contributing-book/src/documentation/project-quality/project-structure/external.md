# External Project

These are projects that expose an interface, e.g., the `ABI` of your contract(s), that can be imported from other projects.

Here is an example structure for such an application that consists of two separate projects. In this example, `my_library` contains the `ABI` of the contract. `my_contract` depends on `my_library` and contains the implementation of the `ABI`. This structure allows `my_library` to be imported from outside `my_application`.

```
my_application/
├── my_library/
└── my_contract/
```

Here, the interface that is exposed for external use is in `my_library` folder.

```
my_library/
├── src/
├──── lib.sw
├── tests/
├── Cargo.toml
└── Forc.toml
```

`my_contract` folder has a structure similar to an [internal project](internal.md), minus the `interface.sw` file, since the interface is now defined in `my_library`.

```
my_contract/
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
