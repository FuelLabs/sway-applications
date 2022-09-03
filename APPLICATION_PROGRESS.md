Table of Content
- [Overview](#overview)
- [Applications](#applications)
  - [Decentralized Apps](#decentralized-apps)
  - [Games](#games)
  - [Tutorials](#tutorials)

# Overview

The content in this document is meant to outline the current state of each project within the repository and future work. The reason is that it's easier for a user to glance at a neatly structured document than to browse through the current issues in order to piece together an overview of the applications.

That being said this document is not meant to perfectly track each issue and instead it's meant to be more of a guidance or a vision of where the projects are heading in order to allow a broad overview of each project and enable easier onboarding for contributors. 

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

- Currently an initial draft PR is under development for the smart contracts

Contracts ❌

- Sway libs has recently added a merkle library which the airdrop requires

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Tests are partially written
  - Need to bump SDK version once the repo catches up
  - SDK has block manipulation so tests can continue to be written

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<details>
<summary>Auctions</summary>

- Currently initial draft PRs exist (smart contracts) for the dutch and english auctions

Contracts

- Blind Auction ❌
  - Not under development at this time

- Dutch Auction ❌
  - Requires vec support in SDK
  - Lots of work to be done for quality in PR

- English Auction ❌
  - Requires vec support in SDK
  - Lots of work to be done for quality in PR

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Blocked by vec for testing
  - No tests written for Dutch, some tests written for English
  - Need to bump SDK version once the repo catches up

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Outdated in PRs, requires updates to readme and a spec should be created

</details>

---

<details>
<summary>Decentralized Autonomous Organization (DAO)</summary>

Contracts ❌

- Replace constructor with manifest instantiation?
- Need to possibly handle overflowing upon calculating votes inside `execute`
- Outdated way to call an arbitrary contract, WIP in Sway repo
- Can instantiate with approval of 1 - exploitable
- Extend to use multiple consensus mechansims instead of a simple yes:no ratio
- Not alphabetically ordered

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Need to bump SDK version once the repo catches up
  - SDK has block manipulation so tests can continue to be written

- Typescript ❌
  - Not under development at this time

Documentation ✅

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

</details>

---

<details>
<summary>Escrow</summary>

Contracts ✅

- Feature complete for UI integration
- Needs some getters so that contracts can interact

User Interface ❌

- Currently under development

Tests

- Rust ✅
  - Need to bump SDK version once the repo catches up

- Typescript ❌
  - Currently under development

Documentation ✅

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

</details>

---

<details>
<summary>Flashloan</summary>

Contracts ❌

- Not under development at this time

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<details>
<summary>Fractionalized Non-Fungible Token (NFT)</summary>

Contracts ❌

- Not under development at this time

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<details>
<summary>Fundraiser</summary>

Contracts ❌

- Rename `contract_abi` to `interface`
- Move documentation onto the interface rather than have it on the implementation
- Campaigns do not have any descriptions / titles / context
  - Should probably use a vec to store data that a human can use to distinguish between campaigns
  - Cannot search for campaigns aside from by a number from 0...X where X is known
- No easy way to retrieve campaigns by user
  - Must iterate from 0...X where X is known by another function call
- Not alphabetically ordered

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - SDK has block manipulation so tests can continue to be written
  - Need to bump SDK version once the repo catches up

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Need to simplify to make it look like the Escrow / DAO spec

</details>

---

<details>
<summary>Liquidity Module</summary>

Contracts ❌

- Not under development at this time

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time
- Issue is not documented

</details>

---

<details>
<summary>Multi-signature Wallet</summary>

Contracts ❌

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

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Needs vec support to test, partial tests written for basic functionality in draft PR
  - Need to bump SDK version once the repo catches up

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Does not exist

</details>

---

<details>
<summary>Name Registry</summary>

Contracts ❌

- Not under development at this time

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<details>
<summary>Non-fungible Token (NFT)</summary>

- Will move from Apps repo to Libs repo soon

Contracts ❌

- Needs vec and option but theu are not supported in the SDK so cannot test

User Interface ✅

- Will not exist for an NFT and instead other applications which integrate the NFT will have their own UI's

Tests

- Rust ❌
  - Need to bump SDK version once the repo catches up
  - Needs to support vec and option to complete testing

- Typescript ✅
  - Will not exist

Documentation ❌

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

Contracts ✅

- Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects
- Need option but SDK may not support it in the latest release so using `u64` for price instead

User Interface ❌

- Not under development at this time

Tests

- Rust ✅
  - Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects

- Typescript ❌
  - Not under development at this time

Documentation ✅

- Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects

</details>

---

<details>
<summary>Over the counter (OTC) Swap Predicate</summary>

Contracts ✅

- Predicate seems to be complete in draft PR

User Interface ❌

- Not under development at this time

Tests

- Rust ✅
  - Tests seem to be complete in draft PR

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Readme ✅
- Spec does not exist ❌

</details>

---

<details>
<summary>Staking</summary>

Contracts ❌

- Draft PR under development
- Lots of work to be done for quality

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Need to bump SDK version once the repo catches up
  - Lots of work needs to be done

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<details>
<summary>Token Vault</summary>

Contracts ❌

- Has been started in a branch a long time ago, not marked as a draft PR
- Basic outline for contract without implementations

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Not under development at this time

</details>

---

<br>

## Games

<details>
<summary>Tic Tac Toe</summary>

Contracts ❌

- Draft PR needs to be updated to a newer release
- Lots of work to be done

User Interface ❌

- Not under development at this time

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Draft PR, Readme and Spec need to be reworked

</details>

---

<br>

## Tutorials

<details>
<summary>Counter</summary>

Contracts ❌

- Draft PR needs to be updated to a newer release

User Interface ❌

- Draft PR needs to be updated to a newer release

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Readme ✅
- Spec does not exist ❌

</details>

---

<details>
<summary>Todo</summary>

Contracts ❌

- Draft PR needs to be updated to a newer release

User Interface ❌

- Draft PR needs to be updated to a newer release

Tests

- Rust ❌
  - Not under development at this time

- Typescript ❌
  - Not under development at this time

Documentation ❌

- Readme ✅
- Spec does not exist ❌

</details>

---
