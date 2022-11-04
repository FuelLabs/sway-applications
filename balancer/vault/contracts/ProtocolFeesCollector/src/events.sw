library events;

pub struct SwapFeePercentageChanged {
    /// new swap fee percentage
    new_swap_fee_percentage: u64,
}

pub struct FlashLoanFeePercentageChanged {
    /// swap fee percentage changed
    new_flash_loan_fee_percentage: u64,
}