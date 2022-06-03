<div id="top"></div>

<div align="center">
  <a href="https://github.com/FuelLabs/sway-applications">
    <img src="../logo.png" alt="Logo" width="700" height="170">
  </a>

  <h3 align="center">React Counter</h3>

  <p align="center">
    Sway Counter Smart Contract with React and Fuel v2 TypeScript SDK
    <br />
    <a target="_blank" href="https://fuellabs.github.io/fuels-ts/"><strong>Explore the Fuel TypeScript SDK docs »</strong></a>
  </p>
</div>

## Table of contents

- [SDK documentation](https://fuellabs.github.io/fuels-ts/)
- [About](#about)
  - [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Walkthrough](#walkthrough)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## About The Project

![Demo Screen Shot](img/preview.png)

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [Sway](https://fuellabs.github.io/sway/latest/)
- [fuels-ts](https://fuellabs.github.io/fuels-ts/)
- [React.js](https://reactjs.org/)
- [TypeChain](https://github.com/dethcrypto/TypeChain)
- [TypeScript](https://www.typescriptlang.org/)

<p align="right">(<a href="#top">back to top</a>)</p>

## Getting Started

We recommend that you first read through and understand the basics of [Sway](https://fuellabs.github.io/sway/latest/), the DSL for the Fuel Virtual Machine.

### Prerequisites

Your machine will need to have a few things installed in order to run a local fuel node and this React application:

- [The latest LTS version of Node.js](https://nodejs.org/)
- [The latest stable Rust toolchain](https://fuellabs.github.io/sway/latest/introduction/installation.html#dependencies)
- [forc and fuel-core binaries](https://fuellabs.github.io/sway/latest/introduction/installation.html)

### Installation

0. Before beginning, verify that you have completed all [Prerequisites](#prerequisites) installations.

1. Clone the repo

```sh
git clone https://github.com/FuelLabs/sway-applications.git
```

2. Open this demo application

```sh
cd react-sway-counter
```

3. Install NPM packages

```sh
npm i
```

### Walkthrough

This application demo consists of four main milestones on your journey to writing your first Sway + React app:

1. Creating a deployable A Sway program - in this example we will create a [Smart Contract](https://fuellabs.github.io/sway/latest/sway-program-types/smart_contracts.html)
2. Compiling your Sway contract
3. Generating TypeScript for Sway contract
4. Deploying your Sway contract
5. Using your Sway contract in a React project

#### 1. Creating a deployable A Sway program

Our demo application uses a counter contract with methods for interacting with a stored counter, based primarily off of this [example Sway counter program](https://fuellabs.github.io/sway/latest/examples/counter.html).

In this demo, our Sway program has these four methods, [see main.sw](src/main.sw):

```rust
fn increment_counter(value: u64) -> u64;
fn decrement_counter(value: u64) -> u64;
fn get_counter() -> u64;
```

#### 2. Compiling your Sway contract

The Sway program source can be compiled into binary and generate an Application Binary Interface (ABI) JSON file that allows typed and strict interoperability between Fuel and development languages like Rust and TypeScript.

Use this command to take the contents of [main.sw](src/main.sw) and compile it.

```sh
forc build
```

#### 3. Generating TypeScript for Sway contract

Once a Sway program has an Application Binary Interface (ABI) JSON file, TypeScript bindings can be created using the Fuel TypeScript SDK.

For convenience this npm script will run `forc build` from the previous step and then run the generated ABI through [TypeChain](https://github.com/dethcrypto/TypeChain) to create TypeScript types and helpers for the contract.

```sh
npm run build-contract
```

#### 4. Deploying your Sway contract

Your compiled Sway contract is now ready to be sent into a running Fuel Node!

When running `fuel-core` locally, it is useful to have it pre-configured using a chain config, one such example can be found on the [fuels-ts repo](https://github.com/FuelLabs/fuels-ts/blob/master/services/fuel-core/chainConfig.json), which includes a `wallet.privateKey` that you can utilize for a `GENESIS_SECRET` in your local `.env` file.

Create and update `.env` file with relevant values, see [.env.example](.env.example)

```
GENESIS_SECRET="<YOUR SECRET>"
PRIVATE_KEY="<YOUR WALLET PRIVATE KEY>" # generate one - see below
FUEL_PROVIDER_URL="<YOUR FUEL CORE URL>" # will often be http://127.0.0.1:4000/graphql
### You will obtain a CONTRACT_ID below
CONTRACT_ID="<YOUR CONTRACT ID>" # see below
```

For your `PRIVATE_KEY`, you can use this helpful script to generate one:

```sh
npm run generate-private-key
```

For your `CONTRACT_ID`, once you have your environment variables ready, you can then run this command to deploy your Sway contract to your local Fuel provider:

```sh
npm run deploy-contract
```

Which will output a new `CONTRACT_ID` to save inside your `.env`. With that, your Sway contract is compiled, deployed, and available via a TypeScript interface.

#### 5. Using your Sway contract in a React project

Your generated Sway TypeScript bindings should be available inside `src/counter-contract-types`, which have a helpful Contract Factory. This is the most intuitive way to utilize a contract in a TypeScript project.

Import the Fuels Wallet class and contract factory:

```javascript
import { Wallet } from "fuels";
import { CounterContractAbi__factory as Factory } from "./counter-contract-types";
```

Create a new Wallet instance using your generated Wallet Private Key, connecting to your local Fuel Node:

```javascript
const wallet = new Wallet(
  process.env.PRIVATE_KEY,
  process.env.FUEL_PROVIDER_URL
);
```

And finally, connect to your contract via the Factory, to obtain an instance of the Contract that you can then interact with using the created methods:

```javascript
const contractInstance = Factory.connect(process.env.CONTRACT_ID, wallet);
```

A snippet of the Sway program methods for convenience:

```rust
fn increment_counter(value: u64) -> u64;
fn decrement_counter(value: u64) -> u64;
fn get_counter() -> u64;
```

And the same methods available in JavaScript:

```javascript
let newValue = await contractInstance.submit.increment_counter();
let newValue = await contractInstance.submit.decrement_counter();
let newValue = await contractInstance.submit.get_counter();
```

In a React app, we can leverage this data access just like any other function:

```javascript
const App = () => {
  const [counterValue, setCounterValue] = useState(-10n);
  useEffect(() => {
    contractInstance.submit.get_counter().then(setCounterValue);
  }, [setCounterValue]);

  return <div>counter value is {counterValue.toString()}</div>;
};
```

To run the demo React application, use this command:

```sh
npm run dev
```

##### What's going on behind the scenes?

When you share your wallet private key and connect to a provider, you are authorizing communication with a Fuel node. The Fuel TypeScript SDK then uses GraphQL and the ABI generated previously to directly work with a deployed contract.

<p align="right">(<a href="#top">back to top</a>)</p>

## Usage

To quickly run this demo application, follow these steps, or see detailed tutorial above

1. Run React application in dev mode, make sure to configure a new `.env` file.

```sh
npm run dev
```

2. View in local browser

```sh
open http://localhost:3000
```

## Contributing

In order to contribute to this demo, please see the main [sway-applications](https://github.com/FuelLabs/sway-applications) repo.

In order to contribute to the `Fuel TypeScript SDK`, please see the main [fuels-ts](https://github.com/FuelLabs/fuels-ts) monorepo.

## License

The primary license for `sway-applications` is `Apache 2.0`, see [LICENSE](../LICENSE).
