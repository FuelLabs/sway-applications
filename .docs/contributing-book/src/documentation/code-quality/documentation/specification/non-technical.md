# Non-technical specification

This type of specification is meant to be a higher level overview of your project and is meant to be communicated to parties that may not be as tech-savy or even understand the technology at all. For example, when introducing someone to blockchain for the first time one might start to explain how real world transactions work followed by what a ledger is and how that data is stored. Thereafter, conversation of a blockchain might follow etc. The idea is that concepts, real world use cases and explanations are presented to a user in order to gently introduce them to the area. One would absolutely not expect to start the conversation with peer-to-peer communication, node infrastructure, how transactions are selected from a mempool etc.

To be a little more pragmatic, the expectation from a document would be to outline the use cases of your application and explain them.
Here are some uses cases:

- A user can deposit funds
  - What funds can they deposit?
    - `Answer:` A user can deposit tokens: X, Y, Z because they have been vetted by the organization
  - How often can they deposit?
    - `Answer:` A user can deposit as often as they like given that no fault has been detected by the organization which resulted in deposits being paused
  - When can they withdraw their funds?
    - `Answer:` There is a 24 lock up period after which the user can withdraw their deposit in full
- A user can lend out a portion of their funds
  - How much can a user lend out?
    - `Answer:` A user can lend out up to 100% of their currently unlocked deposit
- A user can borrow against a portion of their funds
  - How much can a user borrow?
    - `Answer:` A user can borrow up to at most 50% of their deposit in order to maintain a sufficient collateralization ratio after which liquidation may occur

Each of those use cases would be broken down into steps to see all the possibilities and they would all be evaluated.

> *Note*
> Diagrams are often a very helpful tool to aid in explanations within a specification and thus a diagram for each use case is encouraged
