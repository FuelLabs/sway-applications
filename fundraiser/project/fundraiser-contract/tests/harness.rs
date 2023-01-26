// TODO: Until the SDK supports block manipulation changing tests may break them because of the
//       specifically selected block deadlines so your test might be correct but the deadline is
//       messing up the test
//
//  - claim_pledges
//      - revert_when_claiming_before_deadline (need SDK to manipulate block height)
//  - pledges
//      - revert_when_pledging_after_deadline (need SDK to manipulate block height)
//
//      When logging is deserialized in the SDK, check logs are correct

mod functions;
mod utils;
