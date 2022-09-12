# Technical specification

The technical specification is an extension of the non-technical specification. This part goes into all of the details that have been deemed too complex to include in the "introduction" (non-technical specification).

> *NOTE*
> A technical specification includes a lot of detail and thus the following does not provide example answers because that depends on your specific use case, your risk profile and trade-offs.

For example, using some of the bullet points above:

- A user can deposit funds
  - What funds can they deposit?
    - How does the system know to only accept tokens: X, Y, Z?
      - Are the token addresses hardcoded or can someone add them at runtime?
    - Who has the authority to tell the system to include that information?
      - If changes can be made at runtime then who is the "owner" who is authorized to make those changes?
    - Do they have authority to make further additions / remove the current tokens?
      - Is there any mechanism that specifies how many tokens they can add, which tokens they can select, how often they can make changes, which addresses they are able to use in order to interact with the system?
    - What happens when a different token is deposited?
      - Do you revert or transfer the funds back - is there some easter egg where you send back a wrapped version of the asset (who knows, it's your system so why not)?
    - Where are the funds stored when a deposit is made?
      - Does the contract that accepts the deposit custody the funds or are they transferred to some vault that is managed by a DAO?

As you can see, a single question spawns more questions which can spawn more technical questions regarding how contracts will be designed (monolithic vs modular, proxies or no proxies, upgradeable?), implemented (each concept can have many implementations - some better than others), deployed etc.

> *NOTE*
> Diagrams are also important in technical documentation because it may be easier to follow a complex workflow visually than having to remember all of the branching sentences / paths
