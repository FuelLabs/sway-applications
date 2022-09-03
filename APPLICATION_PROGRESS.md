Table of Content
- [Overview](#overview)
- [Applications](#applications)
  - [Decentralized Apps](#decentralized-apps)
  - [Games](#games)
  - [Tutorials](#tutorials)

# Overview

The content in this document is meant to outline the current state of each project within the repository and future work. The reason is that it's easier for a user to glance at a neatly structured document than to browse through the current issues in order to piece together an overview of the applications.

That being said this document is not meant to perfectly track each issue and instead it's meant to be more of a guidance or a vision of where the projects are heading in order to allow a broad overview of each project and enable easier onboarding for contributors. For this reason the only projects that have content in their sections will be the ones that are currently in `master`. The remaining empty projects are placeholders used to indicate that work is planned and possibly in a draft pull request at this time.

For any specific issues we suggest taking a look into the issues and pull requests and using the associated labels to filter by a specific project.

The sections contain one of two icons which are used to indicate the state of completion

- ✅ for "No further work seems to be needed"
- ❌ for "Not considered to be complete"

# Applications

The information in this section is split into subsections in order to conceptually differentiate the intention behind the applications. The applications within each subsection are ordered alphabetically for navigation purposes.

- [Decentralized Apps](#decentralized-apps)
  - The projects in this section are meant to be (complete and) presentable to users in order to show them what the Fuel ecosystem is capable of. This means that the applications in this section require the greatest attention and will likely contain the greatest complexity and the most up to date features.
- [Games](#games)
  - The games section is a mixture of getting started with sway and having complex games function on a blockchain. If you don't know what to build then take a look at any existing games and see if you can improve them, alternatively, come up with a game and code its rules into a smart contract. A user interface or a CLI can be built to allow interaction with your game.
- [Tutorials](#tutorials)
  - The projects in the tutorials are a step down from [decentralized apps](#decentralized-apps) in so far as they are meant to take a new developer through the experience of building on Fuel. It's a place for step-by-step instructions that guide a developer from the creation of their contract to interacting with it via a user interface.

<br>

## Decentralized Apps

<details>
<summary>Airdrop</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Auctions</summary>

<h3>Contracts</h3>

- Blind Auction
- Dutch Auction
- English Auction

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Decentralized Autonomous Organization (DAO)</summary>

<h3>Contracts ❌</h3>

- Replace constructor with manifest instantiation?
- Need to possibly handle overflowing upon calculating votes inside `execute`
- Outdated way to call an arbitrary contract, WIP in Sway repo
- Can instantiate with approval of 1 - exploitable
- Extend to use multiple consensus mechansims instead of a simple yes:no ratio
- Not alphabetically ordered

<h3>User Interface ❌</h3>

- Not under development at this time

<h3>Tests</h3>

- <h3>Rust ❌</h3>
  - Need to bump SDK version once the repo catches up
  - SDK has block manipulation so tests can continue to be written

- <h3>Typescript ❌</h3>
  - Not under development at this time

<h3>Documentation ✅</h3>

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

</details>

---

<details>
<summary>Escrow</summary>

<h3>Contracts ✅</h3>

- Feature complete for UI integration
- Needs some getters so that contracts can interact

<h3>User Interface ❌</h3>

- Currently under development

<h3>Tests</h3>

- <h3>Rust ✅</h3>
  - Need to bump SDK version once the repo catches up

- <h3>Typescript ❌</h3>
  - Currently under development

<h3>Documentation ✅</h3>

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

</details>

---

<details>
<summary>Flashloan</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Fractionalized Non-Fungible Token (NFT)</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Fundraiser</summary>

<h3>Contracts ❌</h3>

- Rename `contract_abi` to `interface`
- Move documentation onto the interface rather than have it on the implementation
- Campaigns do not have any descriptions / titles / context
  - Should probably use a vec to store data that a human can use to distinguish between campaigns
  - Cannot search for campaigns aside from by a number from 0...X where X is known
- No easy way to retrieve campaigns by user
  - Must iterate from 0...X where X is known by another function call
- Not alphabetically ordered

<h3>User Interface ❌</h3>

- Not under development at this time

<h3>Tests</h3>

- <h3>Rust ❌</h3>
  - SDK has block manipulation so tests can continue to be written
  - Need to bump SDK version once the repo catches up

- <h3>Typescript ❌</h3>
  - Not under development at this time

<h3>Documentation ❌</h3>

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Need to simplify to make it look like the Escrow / DAO spec

</details>

---

<details>
<summary>Liquidity Module</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

- Issue is not documented

</details>

---

<details>
<summary>Multi-signature Wallet</summary>

<h3>Contracts ❌</h3>

- Rename `contract_abi` to `interface`
- Move documentation onto the interface rather than have it on the implementation
- Document events
- Not alphabetically ordered
- Move `create_hash` into `utils`
- Move `count_approvals` to `utils` when libraries support storage access
  - The keyword `break` is implemented, uncomment and use in fn
- Needs to use vec instead of arrays but cannot test in SDK
- Only basic functionality is implemented (lots more to do as listed in issues)
  - Cannot make arbitrary calls yet, work is being done in Sway which hopefully resolves this

<h3>User Interface ❌</h3>

- Not under development at this time

<h3>Tests</h3>

- <h3>Rust ❌</h3>
  - Needs vec support to test, partial tests written for basic functionality in draft PR
  - Need to bump SDK version once the repo catches up

- <h3>Typescript ❌</h3>
  - Not under development at this time

<h3>Documentation ❌</h3>

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Does not exist

</details>

---

<details>
<summary>Name Registry</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Non-fungible Token (NFT)</summary>

- Will move from Apps repo to Libs repo soon

<h3>Contracts ❌</h3>

- Needs vec and option but theu are not supported in the SDK so cannot test

<h3>User Interface ✅</h3>

- Will not exist for an NFT and instead other applications which integrate the NFT will have their own UI's

<h3>Tests</h3>

- <h3>Rust ❌</h3>
  - Need to bump SDK version once the repo catches up
  - Needs to support vec and option to complete testing

- <h3>Typescript ✅</h3>
  - Will not exist

<h3>Documentation ❌</h3>

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Doesn't really belong in apps repo and should be moved to libs repo at some point
    - Although it will be used in other apps
- Specification ❌
  - Does not exist

</details>

---

<details>
<summary>Oracle</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Over the counter (OTC) Swap Predicate</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Staking</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Token Vault</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<br>

## Games

<details>
<summary>Tic Tac Toe</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<br>

## Tutorials

<details>
<summary>Counter</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---

<details>
<summary>Todo</summary>

<h3>Contracts</h3>

<h3>User Interface</h3>

<h3>Tests</h3>

- <h3>Rust</h3>
- <h3>Typescript</h3>

<h3>Documentation</h3>

</details>

---
