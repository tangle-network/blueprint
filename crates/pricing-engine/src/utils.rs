use alloy_primitives::U256;
use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};

use crate::error::{PricingError, Result};

/// Decimal places for on-chain pricing amounts (10^9).
/// Matches the contract's scaled price representation where
/// 1 USD = 1_000_000_000 atomic units.
pub const PRICING_SCALE_PLACES: u32 = 9;

/// Pricing scale factor - used to convert decimal prices to integer atoms
pub fn pricing_scale() -> Decimal {
    let scale = 10u128.pow(PRICING_SCALE_PLACES);
    Decimal::from_u128(scale).expect("invalid pricing scale")
}

/// Convert a Decimal value into a scaled U256 amount suitable for ABI encoding.
pub fn decimal_to_scaled_amount(value: Decimal) -> Result<U256> {
    let scaled = (value * pricing_scale()).trunc();
    if scaled.is_sign_negative() {
        return Err(PricingError::Pricing(
            "Negative prices are not supported".to_string(),
        ));
    }

    if scaled.is_zero() {
        return Err(PricingError::Pricing(
            "Zero price not supported; use explicit free-tier logic".to_string(),
        ));
    }

    let int_value = scaled
        .to_u128()
        .ok_or_else(|| PricingError::Pricing("Failed to scale price into u128".to_string()))?;
    Ok(U256::from(int_value))
}

/// Convert an exposure percentage (0-100) into basis points used on-chain.
pub fn percent_to_bps(percent: u32) -> Result<u16> {
    if percent > 100 {
        return Err(PricingError::Pricing(format!(
            "Exposure percent {percent}% exceeds 100%"
        )));
    }
    // Safe: 100 * 100 = 10000, fits u16 (max 65535)
    Ok(u16::try_from(percent * 100).expect("percent <= 100 guarantees fit"))
}
