# External Project

These are projects that expose an interface, e.g., the `ABI` of your contract(s), that can be imported into other projects.

The following structure separates the interface of the contract into its own library project so that it can be imported alongside projects in the `my_application` directory and outside of it.

```
my_application/
├── my_library/
└── my_contract/
```

The interface has a simple structure in this example because it consists of a single file, `lib.sw`.

```
my_library/
├── src/
├──── lib.sw
├── tests/
├── Cargo.toml
└── Forc.toml
```

The contract follows the structure of an [internal project](internal.md) however since the interface is now its own project `interface.sw` has been removed from the `src` directory and it is being imported in the manifest file.

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
