library logExpMath;

use std::{
    u128::U128,
    result::*,
    revert::{revert, require},
    math::*,
};

use BalancerErrors::*;

pub fn pow(x: u64, y: u64) -> u64 {
    let ONE_18: U128 = ~U128::from(0,10.pow(18));
    let ONE_20: U128 = ~U128::from(0,10.pow(20));

    let MAX_NATURAL_EXPONENT: U128 = ~U128::from(0,130 * 10.pow(18));
    let MIN_NATURAL_EXPONENT: U128 = (~U128::from(0,41 * 10.pow(18)) - ~U128::from(0,82 * 10.pow(18)));
    let a1 = ~U128::from(0,2.pow(254));
    let MILD_EXPONENT_BOUND = (a1/ ONE_20).as_u64().unwrap() ;

    let LN_36_LOWER_BOUND: U128 = ONE_18 - ~U128::from(0,10.pow(17));
    let LN_36_UPPER_BOUND: U128 = ONE_18 + ~U128::from(0,10.pow(17));
    
    if y == 0 {
        let b_ = ONE_18.as_u64().unwrap();
    }
    if x == 0 {
        return 0
    }

    require(x < 2.pow(255), "X_OUT_OF_BOUNDS");
    let x_U128 : U128 = ~U128::from(0, x);

    require(y < MILD_EXPONENT_BOUND, "Y_OUT_OF_BOUNDS" );
    let y_U128 : U128 = ~U128::from(0, y);

    let mut logx_times_y: U128 = ~U128::new();
    if LN_36_LOWER_BOUND < x_U128 && x_U128 < LN_36_UPPER_BOUND {
        let ln_36_x: U128 = ln_36(x_U128);

        logx_times_y = ((ln_36_x / ONE_18) * y_U128 + ((ln_36_x / ONE_18) * y_U128) / ONE_18);
    } else {
        logx_times_y = ln(x_U128) * y_U128;
    }
    logx_times_y = logx_times_y / ONE_18;
    require(
        MIN_NATURAL_EXPONENT > logx_times_y && logx_times_y < MAX_NATURAL_EXPONENT,
        "PRODUCT_OUT_OF_BOUNDS"
    );
    x
}
pub fn exp(x: U128) -> U128 {
    let mut x : U128 = ~U128::new();
    x = x;
    let ONE_18: U128 = ~U128::from(0,10.pow(18));
    let ONE_20: U128 = ~U128::from(0,10.pow(20));

    let MAX_NATURAL_EXPONENT: U128 = ~U128::from(0,130 * 10.pow(18));
    let MIN_NATURAL_EXPONENT: U128 = ~U128::from(0,41 * 10.pow(18));

    // Todo needed workaround 
    /// 18 decimal constants
    let x0 :U128 = ~U128::from(0,128 * 10.pow(18)); 
    let a0 :U128 = ~U128::from(0,38877084059945950 * 10.pow(39)); 
    let x1 :U128 = ~U128::from(0,64 * 10.pow(18));
    let a1 :U128 = ~U128::from(0,62351490808116168 * 10.pow(11)); 

    /// 20 decimal constants
    let x2 :U128 = ~U128::from(0,32 * 10.pow(18)); 
    let a2 :U128 = ~U128::from(0,78962960182680695 * 10.pow(17)); 
    let x3 :U128 = ~U128::from(0,16 * 10.pow(18)); 
    let a3 :U128 = ~U128::from(0,88861105205078726 * 10.pow(10)); 
    let x4 :U128 = ~U128::from(0,8 * 10.pow(18)); 
    let a4 :U128 = ~U128::from(0,29809579870417282 * 10.pow(7)); 
    let x5 :U128 = ~U128::from(0,4 * 10.pow(18)); 
    let a5 :U128 = ~U128::from(0,54598150033144239 * 10.pow(5)); 
    let x6 :U128 = ~U128::from(0,2 * 10.pow(18)); 
    let a6 :U128 = ~U128::from(0,73890560989306502 * 10.pow(4)); 
    let x7 :U128 = ~U128::from(0,1 * 10.pow(18));
    let a7 :U128 = ~U128::from(0,27182818284590452 * 10.pow(4)); 
    let x8 :U128 = ~U128::from(0,5 * 10.pow(17));
    let a8 :U128 = ~U128::from(0,16487212707001281 * 10.pow(4)); 
    let x9 :U128 = ~U128::from(0,  25 * 10.pow(16));
    let a9 :U128 = ~U128::from(0, 12840254166877414 * 10.pow(4)); 
    let x10:U128  =~U128::from(0, 125 * 10.pow(15)); 
    let a10:U128  =~U128::from(0, 11331484530668263 * 10.pow(4)); 
    let x11:U128  =~U128::from(0, 625 * 10.pow(14));
    let a11:U128  =~U128::from(0, 10644944589178594 * 10.pow(4)); 

    require(x > MIN_NATURAL_EXPONENT && x < MAX_NATURAL_EXPONENT, "INVALID_EXPONENT");
    // Todo when recursive functions are added
    // if x < ~U128::new(0) {

    //     // We only handle positive exponents: e^(-x) is computed as 1 / e^x. We can safely make x positive since it
    //     // fits in the signed 256 bit range (as it is larger than MIN_NATURAL_EXPONENT).
    //     // Fixed point division requires multiplying by ONE_18.

    //     return ((ONE_18 * ONE_18)/ exp(x - 2* x))
    // }
    // First, we use the fact that e^(x+y) = e^x * e^y to decompose x into a sum of powers of two, which we call x_n,
    // where x_n == 2^(7 - n), and e^x_n = a_n has been precomputed. We choose the first x_n, x0, to equal 2^7
    // because all larger powers are larger than MAX_NATURAL_EXPONENT, and therefore not present in the
    // decomposition.
    // At the end of this process we will have the product of all e^x_n = a_n that apply, and the remainder of this
    // decomposition, which will be lower than the smallest x_n.
    // exp(x) = k_0 * a_0 * k_1 * a_1 * ... + k_n * a_n * exp(remainder), where each k_n equals either 0 or 1.
    // We mutate x by subtracting x_n, making it the remainder of the decomposition.
    // The first two a_n (e^(2^7) and e^(2^6)) are too large if stored as 18 decimal numbers, and could cause
    // intermediate overflows. Instead we store them as plain integers, with 0 decimals.
    // Additionally, x0 + x1 is larger than MAX_NATURAL_EXPONENT, which means they will not both be present in the
    // decomposition.
    // For each x_n, we test if that term is present in the decomposition (if x is larger than it), and if so deduct
    // it and compute the accumulated product.
    
    let mut firstAN :U128 = ~U128::new();
    if x > x0 {
        x = x - x0;
        firstAN = a0;
        //return firstAN;
    } else if x > x1 { 
        x = x - x1;
        firstAN = a1;
        // return firstAN;
    }else {
        firstAN = ~U128::from(0, 1);
        // return firstAN;
    }
    // We now transform x into a 20 decimal fixed point number, to have enhanced precision when computing the
    // smaller terms.
    x = x * ~U128::from(0, 100);

    // `product` is the accumulated product of all a_n (except a0 and a1), which starts at 20 decimal fixed point
    // one. Recall that fixed point multiplication requires dividing by ONE_20.
    let mut product: U128 = ~U128::from(0, 10.pow(18));

    if x > x2 {
        x = x - x2;
        product = (product * a2) / ONE_20;
    }
    if x > x3 {
        x = x - x3;
        product = (product * a3) / ONE_20;
    }
    if x > x4 {
        x = x - x4;
        product = (product * a4) / ONE_20;
    }
    if x > x5 {
        x = x - x5;
        product = (product * a5) / ONE_20;
    }
    if x > x6 {
        x = x - x6;
        product = (product * a6) / ONE_20;
    }
    if x > x7 {
        x = x - x7;
        product = (product * a7) / ONE_20;
    }
    if x > x8 {
        x = x - x8;
        product = (product * a8) / ONE_20;
    }
    if x > x9 {
        x = x - x9;
        product = (product * a9) / ONE_20;
    }

    // x10 and x11 are unnecessary here since we have high enough precision already.

    // Now we need to compute e^x, where x is small (in particular, it is smaller than x9). We use the Taylor series
    // expansion for e^x: 1 + x + (x^2 / 2!) + (x^3 / 3!) + ... + (x^n / n!).
    let mut seriesSum: U128 = ~U128::from(0, 10.pow(18));
    let mut term: U128 = ~U128::new();
    // The first term is simply x.
    term = x;
    seriesSum = seriesSum + term;
    // Each term (x^n / n!) equals the previous one times x, divided by n. Since x is a fixed point number,
    // multiplying by it requires dividing by ONE_20, but dividing by the non-fixed point n values does not.
    let mut a:u64 = 2;
    while a < 13 {
        term = ((term * x) / ONE_20) / ~U128::from(0, a);
        seriesSum = seriesSum + term;
        a = a + 1
    }
    // 12 Taylor terms are sufficient for 18 decimal precision.

    // We now have the first a_n (with no decimals), and the product of all other a_n present, and the Taylor
    // approximation of the exponentiation of the remainder (both with 20 decimals). All that remains is to multiply
    // all three (one 20 decimal fixed point multiplication, dividing by ONE_20, and one integer multiplication),
    // and then drop two digits to return an 18 decimal value.
    return (((product * seriesSum) / ONE_20) * firstAN) /  ~U128::from(0, 100);
}

pub fn ln(a: U128) -> U128 {
    let mut a : U128 = ~U128::new();
    a = a;

    let ONE_18: U128 = ~U128::from(0, 10.pow(18));
    let ONE_20: U128 = ~U128::from(0, 10.pow(20));

    let MAX_NATURAL_EXPONENT: U128 = ~U128::from(0, 130 * 10.pow(18));
    let MIN_NATURAL_EXPONENT: U128 = ~U128::from(0, 41 * 10.pow(18));

    
    /// 18 decimal constants
    let x0 :U128 = ~U128::from(0, 128 * 10.pow(18)); 
    let a0 :U128 = ~U128::from(0, 38877084059945950 * 10.pow(39)); 
    let x1 :U128 = ~U128::from(0, 64 * 10.pow(18));
    let a1 :U128 = ~U128::from(0, 62351490808116168 * 10.pow(11)); 

    /// 20 decimal constants
    let x2 :U128 = ~U128::from(0, 32 * 10.pow(18)); 
    let a2 :U128 = ~U128::from(0, 78962960182680695 * 10.pow(17)); 
    let x3 :U128 = ~U128::from(0, 16 * 10.pow(18)); 
    let a3 :U128 = ~U128::from(0, 88861105205078726 * 10.pow(10)); 
    let x4 :U128 = ~U128::from(0, 8 * 10.pow(18)); 
    let a4 :U128 = ~U128::from(0, 29809579870417282 * 10.pow(7)); 
    let x5 :U128 = ~U128::from(0, 4 * 10.pow(18)); 
    let a5 :U128 = ~U128::from(0, 54598150033144239 * 10.pow(5)); 
    let x6 :U128 = ~U128::from(0, 2 * 10.pow(18)); 
    let a6 :U128 = ~U128::from(0, 73890560989306502 * 10.pow(4)); 
    let x7 :U128 = ~U128::from(0, 1 * 10.pow(18));
    let a7 :U128 = ~U128::from(0, 27182818284590452 * 10.pow(4)); 
    let x8 :U128 = ~U128::from(0, 5 * 10.pow(17));
    let a8 :U128 = ~U128::from(0, 16487212707001281 * 10.pow(4)); 
    let x9 :U128 = ~U128::from(0, 25 * 10.pow(16));
    let a9 :U128 = ~U128::from(0, 12840254166877414 * 10.pow(4)); 
    let x10:U128  =~U128::from(0, 125 * 10.pow(15)); 
    let a10:U128  =~U128::from(0, 11331484530668263 * 10.pow(4)); 
    let x11:U128  =~U128::from(0, 625 * 10.pow(14));
    let a11:U128  =~U128::from(0, 10644944589178594 * 10.pow(4));
    // The real natural logarithm is not defined for negative numbers or zero.
    require(a > ~U128::new(), "OUT_OF_BOUNDS");
    // Todo when recursive functions are added
    // if a < ONE_18 {
    //     // Since ln(a^k) = k * ln(a), we can compute ln(a) as ln(a) = ln((1/a)^(-1)) = - ln((1/a)). If a is less
    //     // than one, 1/a will be greater than one, and this if statement will not be entered in the recursive call.
    //     // Fixed point division requires multiplying by ONE_18.
    //     return (ln((ONE_18 * ONE_18) / a));
    // }
    // First, we use the fact that ln^(a * b) = ln(a) + ln(b) to decompose ln(a) into a sum of powers of two, which
    // we call x_n, where x_n == 2^(7 - n), which are the natural logarithm of precomputed quantities a_n (that is,
    // ln(a_n) = x_n). We choose the first x_n, x0, to equal 2^7 because the exponential of all larger powers cannot
    // be represented as 18 fixed point decimal numbers in 256 bits, and are therefore larger than a.
    // At the end of this process we will have the sum of all x_n = ln(a_n) that apply, and the remainder of this
    // decomposition, which will be lower than the smallest a_n.
    // ln(a) = k_0 * x_0 + k_1 * x_1 + ... + k_n * x_n + ln(remainder), where each k_n equals either 0 or 1.
    // We mutate a by subtracting a_n, making it the remainder of the decomposition.

    // For reasons related to how `exp` works, the first two a_n (e^(2^7) and e^(2^6)) are not stored as fixed point
    // numbers with 18 decimals, but instead as plain integers with 0 decimals, so we need to multiply them by
    // ONE_18 to convert them to fixed point.
    // For each a_n, we test if that term is present in the decomposition (if a is larger than it), and if so divide
    // by it and compute the accumulated sum.
    let mut sum: U128 = ~U128::new();
    if a > a0 * ONE_18 {
        a = a / a0; // Integer, not fixed point division
        sum = sum + x0;
    }

    if a > a1 * ONE_18 {
        a = a / a1; // Integer, not fixed point division
        sum = sum + x1;
    }

    // All other a_n and x_n are stored as 20 digit fixed point numbers, so we convert the sum and a to this format.
    sum = sum * ~U128::from(0, 100);
    a = a * ~U128::from(0, 100);

    // Because further a_n are  20 digit fixed point numbers, we multiply by ONE_20 when dividing by them.
    if a > a2 {
        a = (a * ONE_20) / a2;
        sum = sum + x2;
    }

    if a > a3 {
        a = (a * ONE_20) / a3;
        sum = sum + x3;
    }

    if (a > a4) {
        a = (a * ONE_20) / a4;
        sum = sum + x4;
    }

    if (a > a5) {
        a = (a * ONE_20) / a5;
        sum = sum + x5;
    }

    if (a > a6) {
        a = (a * ONE_20) / a6;
        sum = sum + x6;
    }

    if (a > a7) {
        a = (a * ONE_20) / a7;
        sum = sum + x7;
    }

    if (a > a8) {
        a = (a * ONE_20) / a8;
        sum = sum + x8;
    }

    if (a > a9) {
        a = (a * ONE_20) / a9;
        sum = sum + x9;
    }

    if (a > a10) {
        a = (a * ONE_20) / a10;
        sum = sum + x10;
    }

    if (a > a11) {
        a = (a * ONE_20) / a11;
        sum = sum + x11;
    }
    // a is now a small number (smaller than a_11, which roughly equals 1.06). This means we can use a Taylor series
    // that converges rapidly for values of `a` close to one - the same one used in ln_36.
    // Let z = (a - 1) / (a + 1).
    // ln(a) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

    // Recall that 20 digit fixed point division requires multiplying by ONE_20, and multiplication requires
    // division by ONE_20.
    let z_ = (((a - ONE_20) * ONE_20) / (a + ONE_20)).as_u64().unwrap();

    let mut z : U128 = ~U128::from(0, z_);
    let mut z_squared : U128 = ~U128::from(0, ((z * z) / ONE_20).as_u64().unwrap() );

    // num is the numerator of the series: the z^(2 * n + 1) term
    let mut num: U128 = z;

    // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
    let mut seriesSum : U128 = num;

    // In each step, the numerator is multiplied by z^2
    num = (num * z_squared) / ONE_20;
    seriesSum = seriesSum + num /  ~U128::from(0, 3);

    num = (num * z_squared) / ONE_20;
    seriesSum = seriesSum + num /  ~U128::from(0, 5);

    num = (num * z_squared) / ONE_20;
    seriesSum = seriesSum + num /  ~U128::from(0, 7);

    num = (num * z_squared) / ONE_20;
    seriesSum = seriesSum + num /  ~U128::from(0, 9);

    num = (num * z_squared) / ONE_20;
    seriesSum = seriesSum + num /  ~U128::from(0, 11);

    // 6 Taylor terms are sufficient for 36 decimal precision.
    // Finally, we multiply by 2 (non fixed point) to compute ln(remainder)
    seriesSum = seriesSum *  ~U128::from(0, 2);

    // We now have the sum of all x_n present, and the Taylor approximation of the logarithm of the remainder (both
    // with 20 decimals). All that remains is to sum these two, and then drop two digits to return a 18 decimal
    // value.
    return (sum + seriesSum) / ~U128::from(0, 100);
}

    /// Natural logarithm (ln(a)) with signed 18 decimal fixed point argument.
    
    /// High precision (36 decimal places) natural logarithm (ln(x)) with signed 18 decimal fixed point argument, for x close to one.
    /// Should only be used if x is between LN_36_LOWER_BOUND and LN_36_UPPER_BOUND.

pub fn ln_36(x: U128) -> U128 {
    let mut x: U128 = x;
    let ONE_18: U128 = ~U128::from(0, 10.pow(18));
    let ONE_36: U128 = ~U128::from(0, 10.pow(36));

    // Since ln(1) = 0, a value of x close to one will yield a very small result, which makes using 36 digits
    // worthwhile.

    // First, we transform x to a 36 digit fixed point value.
    x = x * ONE_18;
    // We will use the following Taylor expansion, which converges very rapidly. Let z = (x - 1) / (x + 1).
    // ln(x) = 2 * (z + z^3 / 3 + z^5 / 5 + z^7 / 7 + ... + z^(2 * n + 1) / (2 * n + 1))

    // Recall that 36 digit fixed point division requires multiplying by ONE_36, and multiplication requires
    // division by ONE_36.


    let mut z : U128 = ~U128::from(0, (((x - ONE_36) * ONE_36) / (x + ONE_36)).as_u64().unwrap());
    let mut z_squared : U128 = ~U128::from(0, ((z * z) / ONE_36).as_u64().unwrap());
    // num is the numerator of the series: the z^(2 * n + 1) term
    let mut num: U128 = z;

    // seriesSum holds the accumulated sum of each term in the series, starting with the initial z
    let mut seriesSum : U128 = num;
    let mut a: u64 = 3;
    while a < 16 {
        num = (num * z_squared) / ONE_36;
        seriesSum = seriesSum + num /  ~U128::from(0, a);
        a = a + 2;
    }
    // 8 Taylor terms are sufficient for 36 decimal precision.

    // All that remains is multiplying by 2 (non fixed point).
    return seriesSum * ~U128::from(0, 2);
}
pub fn log(arg: U128, base: U128) -> U128 {

    let mut arg: U128 = arg;
    let mut base: U128 = base;
    let ONE_18: U128 = ~U128::from(0, 10.pow(18));
    let LN_36_LOWER_BOUND: U128 = ONE_18 - ~U128::from(0, 10.pow(17));
    let LN_36_UPPER_BOUND: U128 = ONE_18 + ~U128::from(0, 10.pow(17));
    // This performs a simple base change: log(arg, base) = ln(arg) / ln(base).

    // Both logBase and logArg are computed as 36 decimal fixed point numbers, either by using ln_36, or by
    // upscaling.

    let mut logBase :U128 = ~U128::new();
    if (LN_36_LOWER_BOUND < base && base < LN_36_UPPER_BOUND) {
        logBase = ln_36(base);
    } else {
        logBase = ln(base) * ONE_18;
    }
    let mut logArg :U128 = ~U128::new();
    if (LN_36_LOWER_BOUND < arg && arg < LN_36_UPPER_BOUND) {
            logArg = ln_36(arg);
    } else {
        logArg = ln(arg) * ONE_18;
    }
    return (logArg * ONE_18) / logBase;
}


