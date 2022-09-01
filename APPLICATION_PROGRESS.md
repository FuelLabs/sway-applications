Table of Content
- [Overview](#overview)
- [Applications](#applications)
  - [Decentralized Apps](#decentralized-apps)
    - [Airdrop](#airdrop)
      - [Contracts ❌](#contracts-)
      - [User Interface ❌](#user-interface-)
      - [Tests](#tests)
        - [Rust ❌](#rust-)
        - [Typescript ❌](#typescript-)
      - [Documentation ❌](#documentation-)
    - [Auctions](#auctions)
      - [Contracts](#contracts)
        - [Blind Auction ❌](#blind-auction-)
        - [Dutch Auction ❌](#dutch-auction-)
        - [English Auction ❌](#english-auction-)
      - [User Interface ❌](#user-interface--1)
      - [Tests](#tests-1)
        - [Rust ❌](#rust--1)
        - [Typescript ❌](#typescript--1)
      - [Documentation ❌](#documentation--1)
    - [Decentralized Autonomous Organization (DAO)](#decentralized-autonomous-organization-dao)
      - [Contracts ❌](#contracts--1)
      - [User Interface ❌](#user-interface--2)
      - [Tests](#tests-2)
        - [Rust ❌](#rust--2)
        - [Typescript ❌](#typescript--2)
      - [Documentation ✅](#documentation--2)
    - [Escrow](#escrow)
      - [Contracts ✅](#contracts--2)
      - [User Interface ❌](#user-interface--3)
      - [Tests](#tests-3)
        - [Rust ✅](#rust--3)
        - [Typescript ❌](#typescript--3)
      - [Documentation ✅](#documentation--3)
    - [Flashloan](#flashloan)
      - [Contracts ❌](#contracts--3)
      - [User Interface ❌](#user-interface--4)
      - [Tests](#tests-4)
        - [Rust ❌](#rust--4)
        - [Typescript ❌](#typescript--4)
      - [Documentation ❌](#documentation--4)
    - [Fractionalized Non-Fungible Token (NFT)](#fractionalized-non-fungible-token-nft)
      - [Contracts ❌](#contracts--4)
      - [User Interface ❌](#user-interface--5)
      - [Tests](#tests-5)
        - [Rust ❌](#rust--5)
        - [Typescript ❌](#typescript--5)
      - [Documentation ❌](#documentation--5)
    - [Fundraiser](#fundraiser)
      - [Contracts ❌](#contracts--5)
      - [User Interface ❌](#user-interface--6)
      - [Tests](#tests-6)
        - [Rust ❌](#rust--6)
        - [Typescript ❌](#typescript--6)
      - [Documentation ❌](#documentation--6)
    - [Liquidity Module](#liquidity-module)
      - [Contracts ❌](#contracts--6)
      - [User Interface ❌](#user-interface--7)
      - [Tests](#tests-7)
        - [Rust ❌](#rust--7)
        - [Typescript ❌](#typescript--7)
      - [Documentation ❌](#documentation--7)
    - [Multi-signature Wallet](#multi-signature-wallet)
      - [Contracts ❌](#contracts--7)
      - [User Interface ❌](#user-interface--8)
      - [Tests](#tests-8)
        - [Rust ❌](#rust--8)
        - [Typescript ❌](#typescript--8)
      - [Documentation ❌](#documentation--8)
    - [Name Registry](#name-registry)
      - [Contracts ❌](#contracts--8)
      - [User Interface ❌](#user-interface--9)
      - [Tests](#tests-9)
        - [Rust ❌](#rust--9)
        - [Typescript ❌](#typescript--9)
      - [Documentation ❌](#documentation--9)
    - [Non-fungible Token (NFT)](#non-fungible-token-nft)
      - [Contracts ❌](#contracts--9)
      - [User Interface ✅](#user-interface--10)
      - [Tests](#tests-10)
        - [Rust ❌](#rust--10)
        - [Typescript ✅](#typescript--10)
      - [Documentation ❌](#documentation--10)
    - [Oracle](#oracle)
      - [Contracts ✅](#contracts--10)
      - [User Interface ❌](#user-interface--11)
      - [Tests](#tests-11)
        - [Rust ✅](#rust--11)
        - [Typescript ❌](#typescript--11)
      - [Documentation ✅](#documentation--11)
    - [Over the counter (OTC) Swap Predicate](#over-the-counter-otc-swap-predicate)
      - [Contracts ✅](#contracts--11)
      - [User Interface ❌](#user-interface--12)
      - [Tests](#tests-12)
        - [Rust ✅](#rust--12)
        - [Typescript ❌](#typescript--12)
      - [Documentation ❌](#documentation--12)
    - [Staking](#staking)
      - [Contracts ❌](#contracts--12)
      - [User Interface ❌](#user-interface--13)
      - [Tests](#tests-13)
        - [Rust ❌](#rust--13)
        - [Typescript ❌](#typescript--13)
      - [Documentation ❌](#documentation--13)
    - [Token Vault](#token-vault)
      - [Contracts ❌](#contracts--13)
      - [User Interface ❌](#user-interface--14)
      - [Tests](#tests-14)
        - [Rust ❌](#rust--14)
        - [Typescript ❌](#typescript--14)
      - [Documentation ❌](#documentation--14)
  - [Games](#games)
    - [Tic Tac Toe](#tic-tac-toe)
      - [Contracts ❌](#contracts--14)
      - [User Interface ❌](#user-interface--15)
      - [Tests](#tests-15)
        - [Rust ❌](#rust--15)
        - [Typescript ❌](#typescript--15)
      - [Documentation ❌](#documentation--15)
  - [Tutorials](#tutorials)
    - [Counter](#counter)
      - [Contracts ❌](#contracts--15)
      - [User Interface ❌](#user-interface--16)
      - [Tests](#tests-16)
        - [Rust ❌](#rust--16)
        - [Typescript ❌](#typescript--16)
      - [Documentation ❌](#documentation--16)
    - [Todo](#todo)
      - [Contracts ❌](#contracts--16)
      - [User Interface ❌](#user-interface--17)
      - [Tests](#tests-17)
        - [Rust ❌](#rust--17)
        - [Typescript ❌](#typescript--17)
      - [Documentation ❌](#documentation--17)

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

## Decentralized Apps

### Airdrop

- Currently an initial draft PR is under development for the smart contracts

#### Contracts ❌

- Sway libs has recently added a merkle library which the airdrop requires

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Tests are partially written
- Need to bump SDK version once the repo catches up
- SDK has block manipulation so tests can continue to be written

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

---

### Auctions

- Currently initial draft PRs exist (smart contracts) for the dutch and english auctions

#### Contracts

##### Blind Auction ❌

- Not under development at this time

##### Dutch Auction ❌

- Requires vec support in SDK
- Lots of work to be done for quality in PR

##### English Auction ❌

- Requires vec support in SDK
- Lots of work to be done for quality in PR

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Blocked by vec for testing
- No tests written for Dutch, some tests written for English
- Need to bump SDK version once the repo catches up

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Outdated in PRs, requires updates to readme and a spec should be created

---

### Decentralized Autonomous Organization (DAO)

#### Contracts ❌

- Replace constructor with manifest instantiation?
- Need to possibly handle overflowing upon calculating votes inside `execute`
- Outdated way to call an arbitrary contract, WIP in Sway repo
- Can instantiate with approval of 1 - exploitable
- Extend to use multiple consensus mechansims instead of a simple yes:no ratio
- Not alphabetically ordered

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Need to bump SDK version once the repo catches up
- SDK has block manipulation so tests can continue to be written

##### Typescript ❌

- Not under development at this time

#### Documentation ✅

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

---

### Escrow

#### Contracts ✅

- Feature complete for UI integration
- Needs some getters so that contracts can interact

#### User Interface ❌

- Currently under development

#### Tests

##### Rust ✅

- Need to bump SDK version once the repo catches up

##### Typescript ❌

- Currently under development

#### Documentation ✅

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ✅

---

### Flashloan

#### Contracts ❌

- Not under development at this time

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

---

### Fractionalized Non-Fungible Token (NFT)

#### Contracts ❌

- Not under development at this time

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

---

### Fundraiser

#### Contracts ❌

- Rename `contract_abi` to `interface`
- Move documentation onto the interface rather than have it on the implementation
- Campaigns do not have any descriptions / titles / context
  - Should probably use a vec to store data that a human can use to distinguish between campaigns
  - Cannot search for campaigns aside from by a number from 0...X where X is known
- No easy way to retrieve campaigns by user
  - Must iterate from 0...X where X is known by another function call
- Not alphabetically ordered

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- SDK has block manipulation so tests can continue to be written
- Need to bump SDK version once the repo catches up

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Need to simplify to make it look like the Escrow / DAO spec

---

### Liquidity Module

#### Contracts ❌

- Not under development at this time

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time
- Issue is not documented

---

### Multi-signature Wallet

#### Contracts ❌

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

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Needs vec support to test, partial tests written for basic functionality in draft PR
- Need to bump SDK version once the repo catches up

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Once UI is added it needs to be documented
- Specification ❌
  - Does not exist

---

### Name Registry

#### Contracts ❌

- Not under development at this time

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

---

### Non-fungible Token (NFT)

- Will move from Apps repo to Libs repo soon

#### Contracts ❌

- Needs vec and option but theu are not supported in the SDK so cannot test

#### User Interface ✅

- Will not exist for an NFT and instead other applications which integrate the NFT will have their own UI's

#### Tests

##### Rust ❌

- Need to bump SDK version once the repo catches up
- Needs to support vec and option to complete testing

##### Typescript ✅

- Will not exist

#### Documentation ❌

- Readme ❌
  - Need to remove "current state of app" since this document covers that content
  - Doesn't really belong in apps repo and should be moved to libs repo at some point
    - Although it will be used in other apps
- Specification ❌
  - Does not exist

---

### Oracle

#### Contracts ✅

- Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects
- Need option but SDK may not support it in the latest release so using `u64` for price instead

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ✅

- Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects

##### Typescript ❌

- Not under development at this time

#### Documentation ✅

- Ready to go into master after repo is bumped to latest releases which solve some bugs in other projects

---

### Over the counter (OTC) Swap Predicate

#### Contracts ✅

- Predicate seems to be complete in draft PR

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ✅

- Tests seem to be complete in draft PR

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Readme ✅
- Spec does not exist ❌

---

### Staking

#### Contracts ❌

- Draft PR under development
- Lots of work to be done for quality

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Need to bump SDK version once the repo catches up
- Lots of work needs to be done

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

---

### Token Vault

#### Contracts ❌

- Has been started in a branch a long time ago, not marked as a draft PR
- Basic outline for contract without implementations

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Not under development at this time

## Games

### Tic Tac Toe

#### Contracts ❌

- Draft PR needs to be updated to a newer release
- Lots of work to be done

#### User Interface ❌

- Not under development at this time

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Draft PR, Readme and Spec need to be reworked

## Tutorials

### Counter

#### Contracts ❌

- Draft PR needs to be updated to a newer release

#### User Interface ❌

- Draft PR needs to be updated to a newer release

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Readme ✅
- Spec does not exist ❌

---

### Todo

#### Contracts ❌

- Draft PR needs to be updated to a newer release

#### User Interface ❌

- Draft PR needs to be updated to a newer release

#### Tests

##### Rust ❌

- Not under development at this time

##### Typescript ❌

- Not under development at this time

#### Documentation ❌

- Readme ✅
- Spec does not exist ❌
