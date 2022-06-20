Table of Content
- [Overview](#overview)
- [Use Cases](#use-cases)
  - [Actions that users are able to perform](#actions-that-users-are-able-to-perform)
    - [Campaign Creation](#campaign-creation)
    - [Campaign Cancellation](#campaign-cancellation)
    - [Campaign Claiming](#campaign-claiming)
    - [Pledging to Campaigns](#pledging-to-campaigns)
    - [Unpledging from a Campaign](#unpledging-from-a-campaign)
  - [Information that is presented to users](#information-that-is-presented-to-users)
    - [Author](#author)
    - [User](#user)
    - [Discovery of Campaigns](#discovery-of-campaigns)
    - [Misc](#misc)
- [Specification](#specification)
  - [Smart Contract](#smart-contract)
    - [Creating a campaign](#creating-a-campaign)
      - [Context](#context)
      - [Requirements](#requirements)
    - [Pledging to a campaign](#pledging-to-a-campaign)
      - [Context](#context-1)
      - [Requirements](#requirements-1)
    - [Unpledging from a campaign](#unpledging-from-a-campaign-1)
      - [Context](#context-2)
      - [Requirements](#requirements-2)
    - [Claiming pledges](#claiming-pledges)
      - [Context](#context-3)
      - [Requirements](#requirements-3)
    - [Cancelling a campaign](#cancelling-a-campaign)
      - [Context](#context-4)
      - [Requirements](#requirements-4)
    - [Helper functions for the user interface](#helper-functions-for-the-user-interface)
      - [Campaign Info](#campaign-info)
        - [Context](#context-5)
        - [Requirements](#requirements-5)
      - [Amount pledged by a user](#amount-pledged-by-a-user)
        - [Context](#context-6)
        - [Requirements](#requirements-6)
      - [Campaigns created by author](#campaigns-created-by-author)
        - [Context](#context-7)
        - [Requirements](#requirements-7)
      - [Campaign count](#campaign-count)
        - [Context](#context-8)
        - [Requirements](#requirements-8)
      - [Updating the campaign state](#updating-the-campaign-state)
        - [Context](#context-9)
        - [Requirements](#requirements-9)
  - [User interface](#user-interface)

# Overview

This document provides an overview of the application.

It outline the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

## Actions that users are able to perform

This sub-section details what a user is able to do e.g. click a button and "x, y, z" happens.

### Campaign Creation

A user should be able to create a campaign which consists of

1. The asset that the campaign accepts
2. The amount of asset required to deem the campaign a success a.k.a the goal
3. A deadline after which the campaign is locked and deemed as concluded
4. The beneficiary to whom the asset will be sent to upon reaching the goal

### Campaign Cancellation

The author of a campaign should be able to cancel (end) the campaign

1. If the campaign has not reached its deadline
2. If the campaign has not been cancelled before
3. If the campaign has not been claimed

### Campaign Claiming

The author of a campaign should be able to claim the total pledged amount

1. After the deadline has been passed
2. If the target amount (goal) has been reached
3. If they have not claimed before
4. If they have not cancelled before

### Pledging to Campaigns

A user should be able to pledge to any campaign

1. If the campaign is active (not passed the deadline, cancelled or claimed)
2. If they send the correct asset to the campaign

### Unpledging from a Campaign

A user should be able to unpledge any amount that they have pledged

1. If the campaign has not been claimed

## Information that is presented to users

This sub-section details the information that a user should have access to / what the application provides to them e.g. a history of their previous actions.

### Author

An author should be able to see a history of the campaigns that they have created

1. This should be categorized into currently active and completed campaigns
2. An active campaign is one that has not reached its deadline nor has been cancelled by the author
3. The author should see 
   1. When the campaign ends / time until the deadline
   2. Which campaigns have been cancelled / claimed
   3. The state of the campaign i.e. whether the campaign has succeeded in reaching its goal
      1. Pending state is when the deadline has not been reached
      2. Successful state is when the deadline is reached and the goal has been reached
      3. Failed state is when the deadline is reached and the goal has not been reached
      4. Cancelled state is when the author has cancelled the campaign
   4. The amount pledged by all users and how much is needed to reach the goal
   5. Who the beneficiary is
   6. Which asset the campaign accepts

### User

A user should be able to see the campaigns that they have pledged towards

1. This includes the amount that they have pledged
2. The campaigns should be categorized into active and completed campaigns
3. Only the user should be able to see how much they have pledged

### Discovery of Campaigns

Authors of campaigns and users should be able to share / find campaigns

1. Campaigns should be searchable via the address of the author
   1. `Some category / identifier too ?`

> **NOTE** \
> TODO: how is this information presented to users, is there some main page of all campaigns?

### Misc

1. Track each asset across all campaigns to see how popular each asset is
2. Show total number of campaigns created

> **NOTE** \
> TODO: figure out where to put this info and what else to add

# Specification

## Smart Contract

### Creating a campaign

#### Context

Funds can only be raised if a data structure representing a campaign exists as a target to send funds to.

#### Requirements

- The following parameters are required
  - Asset the campaign accepts for funding
    - `type:` ContractId
    - `restrictions:` 
      - Cannot be the base asset (_0x0000..._)
  - Beneficiary who the funds will go to if the campaign is successful
    - `type:` Identity
    - `description:` Raising funds is valid for both outputs and contracts
  - Deadline after which the campaign is deemed to have ended
    - `type:` u64
    - `description:` Block height will be used as a timer
    - `restrictions:` 
      - Must be in the future (_current block height < deadline_)
  - Target amount of asset the campaign must reach in order to be successful
    - `type:` u64
    - `restrictions:`
      - Must be greater than 0
- The campaign must also track
  - The author (who created the campaign)
    - `type:` Identity
  - Whether the author has claimed the amount upon successful campaign
    - `type:` bool
  - Current pledge amount by everyone
    - `type:` u64
    - `description:` Used to check if the goal has been reached
  - State of the campaign
    - `type:` enum
    - `description:` It should have multiple states to indicate the progress of the campaign
      - Funding: The campaign has been created and the deadline has not been passed nor has the campaign been cancelled
      - Cancelled: The campaign has been cancelled prematurely by the author
      - Successful: The deadline has been passed and the total pledge exceeded the target amount
      - Failed: The deadline has been passed and the total pledge has not reached the target amount
- Anyone can call the function to creates a campaign
- Each author should have a new campaign added to their active campaign list
  - TODO: where to put the following "Campaigns that go past the deadline must be moved from "active" to "completed""
- When a new campaign is created a log should be emitted containing the ID of the campaign and the campaign data structure

### Pledging to a campaign

#### Context

In order to have a campaign move towards its target amount (goal) users need to be able to pledge to the campaign.

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Anyone should be able to pledge any amount of the specified asset in the campaign
- A user can only pledge if the campaign is in the Funding phase and not past the deadline
- The asset being pledgd to the campaign has to be the one the campaign accepts
- Track how much each user has pledged so that they can unpledge
- Log the campaign id and amount pledged


### Unpledging from a campaign

#### Context

If a user has pledged they should have the option to remove their pledge under certain conditions

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
  - Amount to unpledge
    - `type:` u64
- Anyone who has pledged should be able to unpledge
- Users cannot unpledge if the campaign has reached the Success phase
- If a user attempts to unpledge more than they have pledged for the campaign then the amount should be lowered to the total amount that they have pledged in the campaign
- Transfer back their pledge to them
- Log the campaign id and amount unpledged

### Claiming pledges

#### Context

If a campaign has succeeded then the author of the campaign should be able to make a request to claim the funds

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to claim
- Campaign must be in the Success phase
- Author can only claim once
- Campaign should be updated to no longer be active
- Transfer the total amount pledged to the beneficiary
- Log the campaign id

### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface
