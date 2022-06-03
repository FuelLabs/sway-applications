<div id="top"></div>

<div align="center">
  <a href="https://github.com/FuelLabs/sway-applications">
    <img src="../../logo.png" alt="Logo" width="700" height="170">
  </a>

  <h3 align="center">React Todo</h3>

  <p align="center">
    Sway Todo List Smart Contract with React and Fuel v2 TypeScript SDK
    <br />
    <a  href="https://fuellabs.github.io/fuels-ts/"><strong>Explore the Fuel TypeScript SDK docs »</strong></a>
    <br />
    Other Example Apps:
    <br />
    <a href="https://github.com/FuelLabs/swayswap"><strong>⚡️ SwaySwap ⚡️</strong></a>
    <br />
    <a href="https://github.com/FuelLabs/block-explorer-v2"><strong>Block explorer frontend for Fuel</strong></a>
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

## About

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

This application continues building on top of the baseline [react counter application](../counter/) app. Please see that example before continuing.

### Prerequisites

Your machine will need to have a few things installed in order to run a local fuel node and this React application:

- [The latest LTS version of Node.js](https://nodejs.org/)
- [The latest stable Rust toolchain](https://fuellabs.github.io/sway/latest/introduction/installation.html#dependencies)
- [forc and fuel-core binaries](https://fuellabs.github.io/sway/latest/introduction/installation.html)

### Installation

0. Before beginning, verify that you have completed all [Prerequisite](#prerequisites) installations.

1. Clone the repo

```sh
git clone https://github.com/FuelLabs/sway-applications.git
```

2. Open this demo application

```sh
cd tutorials/todo
```

3. Install NPM packages

```sh
npm i
```

### Walkthrough

🛑 Note: This application builds on top of ideas from the [react counter application](../counter/) example. 🛑

In this tutorial, we will complete the following tasks:

1. Creating a deployable Sway [Smart Contract](https://fuellabs.github.io/sway/latest/sway-program-types/smart_contracts.html) capable of more advanced features
2. Compiling your Sway contract, Generating TypeScript, and Deploying
3. Using your Sway contract in a React project

#### 1. Creating a deployable Sway Smart Contract

Our demo application uses a todo contract with methods for interacting with a stored todo list, read more about [contract storage here](https://fuellabs.github.io/sway/latest/blockchain-development/storage.html).

In this demo, our Sway program has these methods, [see main.sw](src/main.sw):

```rust
fn get_todos() -> [Todo; 5];
fn add_todo(index: u8, value: str[20]) -> [Todo; 5];
fn toggle_todo(index: u8) -> [Todo; 5];
fn remove_todo(index: u8) -> [Todo; 5];
```

This program utilizes a Sway struct, allowing storage of more advanced data structures.

```rust
struct Todo {
    completed: bool,
    value: str[20],
}
```

#### 2. Compiling your Sway contract, Generating TypeScript, and Deploying

Using the provided npm scripts, TypeScript bindings can be created using the Fuel TypeScript SDK. Read more about compilation and deployment in the [react counter application](../counter/#2-compiling-your-sway-contract) tutorial.

```sh
npm run build-contract
```

The generated TypeScript will be available in `src/todo-contract-types`.

Create and update `.env` file with relevant values, (see [.env.example](.env.example)):

```
GENESIS_SECRET="<YOUR SECRET>"
PRIVATE_KEY="<YOUR WALLET PRIVATE KEY>"
FUEL_PROVIDER_URL="<YOUR FUEL CORE URL>"
### You will obtain a CONTRACT_ID below
CONTRACT_ID="<YOUR CONTRACT ID>" # see below
```

For `CONTRACT_ID`, once you have your environment variables ready, run this command to deploy your Sway contract to your local Fuel provider:

```sh
npm run deploy-contract
```

This will output a new `CONTRACT_ID` to save inside your `.env`. With that, your Sway contract is compiled, deployed, and available via a TypeScript interface.

#### 3. Using your Sway contract in a React project

Your generated Sway TypeScript bindings should be available inside `src/todo-contract-types`, which have a Contract Factory. Import the Fuels Wallet class and contract factory:

```javascript
import { Wallet } from "fuels";
import { TodoContractAbi__factory as Factory } from "./todo-contract-types";
```

Create a new Wallet instance using your Wallet Private Key, connecting to your local Fuel Node:

```javascript
const wallet = new Wallet(
  process.env.PRIVATE_KEY,
  process.env.FUEL_PROVIDER_URL
);
```

Finally, connect to your contract via the Factory, to obtain an instance of the Contract that you can then interact with using the created methods:

```javascript
const contractInstance = Factory.connect(process.env.CONTRACT_ID, wallet);
```

The contract instance has access to all of Smart Contracts methods created earlier. A snippet of the Sway program methods for convenience:

```rust
fn get_todos() -> [Todo; 5];
fn add_todo(index: u8, value: str[20]) -> [Todo; 5];
fn toggle_todo(index: u8) -> [Todo; 5];
fn remove_todo(index: u8) -> [Todo; 5];
```

And the same methods available in JavaScript:

```javascript
let newValue = await contractInstance.submit.get_todos();
let newValue = await contractInstance.submit.add_todo(0, "take out trash");
let newValue = await contractInstance.submit.toggle_todo(1);
let newValue = await contractInstance.submit.remove_todo(1);
```

In a React app, we can leverage this method access just like any other function:

```javascript
import { Todo } from "~/todo-contract-types/TodoContractAbi";

const App = () => {
  const [todos, setTodos] = useState([]);
  useEffect(() => {
    contractInstance.submit.get_todos().then(setTodos);
  }, [setTodos]);

  return (
    <ul>
      {todos.map((todo) => (
        <li>{todo.value}</li>
      ))}
    </ul>
  );
};
```

To run the demo React application, use this command:

```sh
npm run dev
```

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
