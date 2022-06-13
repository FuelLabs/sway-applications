Table of Content
- [Overview](#overview)
- [Specification](#specification)
  - [1. Smart Contract](#1-smart-contract)
    - [1.1. Creating a campaign](#11-creating-a-campaign)
      - [Context](#context)
      - [Requirements](#requirements)
    - [1.2. Pledging to a campaign](#12-pledging-to-a-campaign)
      - [Context](#context-1)
      - [Requirements](#requirements-1)
    - [1.3. Unpledging from a campaign](#13-unpledging-from-a-campaign)
      - [Context](#context-2)
      - [Requirements](#requirements-2)
    - [1.4. Claiming pledges](#14-claiming-pledges)
      - [Context](#context-3)
      - [Requirements](#requirements-3)
    - [1.5. Cancelling a campaign](#15-cancelling-a-campaign)
      - [Context](#context-4)
      - [Requirements](#requirements-4)
    - [1.6. Helper functions for the user interface](#16-helper-functions-for-the-user-interface)
      - [1.6.1. Campaign Info](#161-campaign-info)
        - [Context](#context-5)
        - [Requirements](#requirements-5)
      - [1.6.2. Amount pledged by a user](#162-amount-pledged-by-a-user)
        - [Context](#context-6)
        - [Requirements](#requirements-6)
      - [1.6.3. Campaigns created by author](#163-campaigns-created-by-author)
        - [Context](#context-7)
        - [Requirements](#requirements-7)
      - [1.6.4. Campaign count](#164-campaign-count)
        - [Context](#context-8)
        - [Requirements](#requirements-8)
      - [1.6.5. Updating the campaign state](#165-updating-the-campaign-state)
        - [Context](#context-9)
        - [Requirements](#requirements-9)
  - [User interface](#user-interface)

# Overview

# Specification

## 1. Smart Contract

### 1.1. Creating a campaign

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

### 1.2. Pledging to a campaign

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


### 1.3. Unpledging from a campaign

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

### 1.4. Claiming pledges

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

### 1.5. Cancelling a campaign

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

### 1.6. Helper functions for the user interface

#### 1.6.1. Campaign Info

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

#### 1.6.2. Amount pledged by a user

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


#### 1.6.3. Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following parameters are required
  - Author identity
    - `type`: Identity
- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### 1.6.4. Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### 1.6.5. Updating the campaign state

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
