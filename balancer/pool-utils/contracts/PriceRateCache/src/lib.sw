library PriceRateCache;

use std::{
    revert::require
};
use BalancerErrors::{
    PRICE_RATE_OVERFLOW
};
use WordCodec::{
    decode_uint,
    encode_uint
};
/*
* Price rate caches are used to avoid querying the price rate for a token every time we need to work with it. It is
* useful for slow changing rates, such as those that arise from interest-bearing tokens (e.g. waDAI into DAI).
*
* The cache data is packed into a single bytes32 value with the following structure:
* [   expires   | duration | price rate value ]
* [   uint64    |  uint64  |      uint128     ]
* [ MSB                                   LSB ]
*
*
* 'rate' is an 18 decimal fixed point number, supporting rates of up to ~3e20. 'expires' is a Unix timestamp, and
* 'duration' is expressed in seconds.
*/

const _PRICE_RATE_CACHE_VALUE_OFFSET: u64 = 0;
const _PRICE_RATE_CACHE_DURATION_OFFSET: u64 = 128;
const _PRICE_RATE_CACHE_EXPIRES_OFFSET: u64 = 128 + 64;

/*
    * Returns the rate of a price rate cache.
    */
pub fn get_rate(cache: b256) ->u64 {
    return decode_uint(cache, _PRICE_RATE_CACHE_VALUE_OFFSET, 128);
}

/*
    * Returns the duration of a price rate cache.
    */
pub fn get_duration(cache: b256) ->u64 {
    return decode_uint(cache, _PRICE_RATE_CACHE_DURATION_OFFSET, 64);
}

/*
    * Returns the duration and expiration time of a price rate cache.
    */
pub fn get_timestamps(cache: b256) ->(u64, u64) {
    let duration = get_duration(cache);
    let expires = decode_uint(cache, _PRICE_RATE_CACHE_EXPIRES_OFFSET, 64);
    return (duration, expires)
}

/*
    * Encodes rate and duration into a price rate cache. The expiration time is computed automatically, counting
    * from the current time.
    */
pub fn encode(rate: u64, duration: u64) ->b256 {
    require(rate >> 128 == 0, PRICE_RATE_OVERFLOW);
    let block_timestamp = 1;
    // solhint-disable not-rely-on-time
    return
        encode_uint(rate, _PRICE_RATE_CACHE_VALUE_OFFSET, 128) |
        encode_uint(duration, _PRICE_RATE_CACHE_DURATION_OFFSET, 64) |
        encode_uint(block_timestamp + duration, _PRICE_RATE_CACHE_EXPIRES_OFFSET, 64);
}

/*
    * Returns rate, duration and expiration time of a price rate cache.
    */
pub fn decode(cache: b256) ->(u64, u64, u64) {
    let rate = get_rate(cache);
    let (duration, expires) = get_timestamps(cache);
    return (rate, duration, expires)
}
