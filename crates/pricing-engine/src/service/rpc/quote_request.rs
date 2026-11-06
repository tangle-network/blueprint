use crate::pricing_engine::GetJobPriceRequest;
use alloy_primitives::{U256, U512};
use blueprint_tangle_extra::job_quote::{Confidentiality, GpuResourceCommitment};
use rust_decimal::Decimal;
use tonic::Status;

pub(super) fn policy(req: &GetJobPriceRequest) -> Result<Confidentiality, Status> {
    let value = req.confidentiality.unwrap_or(u32::from(req.require_tee));
    if req.require_tee && value != u32::from(u8::from(Confidentiality::TeeRequired)) {
        return Err(Status::invalid_argument(
            "confidentiality cannot downgrade require_tee",
        ));
    }
    let value = u8::try_from(value)
        .map_err(|_| Status::invalid_argument("invalid confidentiality policy"))?;
    Confidentiality::try_from(value)
        .map_err(|_| Status::invalid_argument("invalid confidentiality policy"))
}

pub(super) fn gpu(req: &GetJobPriceRequest) -> Result<Option<GpuResourceCommitment>, Status> {
    if req.gpu_class.is_empty() && req.gpu_count == 0 && req.duration_secs == 0 {
        return Ok(None);
    }
    GpuResourceCommitment::new(&req.gpu_class, req.gpu_count, req.duration_secs)
        .map(Some)
        .map_err(|e| Status::invalid_argument(e.to_string()))
}

/// Exact canonical wire units; sub-USD18 amounts are rejected rather than rounded.
pub(super) fn usd18(rate: Decimal) -> Result<Vec<u8>, Status> {
    if rate.is_sign_negative() {
        return Err(Status::failed_precondition(
            "resource price must be nonnegative",
        ));
    }
    let mantissa = rate.mantissa().unsigned_abs();
    let amount = if rate.scale() <= 18 {
        U256::from(mantissa) * U256::from(10u128.pow(18 - rate.scale()))
    } else {
        let divisor = 10u128.pow(rate.scale() - 18);
        if !mantissa.is_multiple_of(divisor) {
            return Err(Status::failed_precondition(
                "resource price is below USD18 precision",
            ));
        }
        U256::from(mantissa / divisor)
    };
    Ok(amount.to_be_bytes::<32>().to_vec())
}

pub(super) fn premium(price: U256, multiplier: Decimal) -> Result<U256, Status> {
    if multiplier.is_sign_negative() {
        return Err(Status::failed_precondition(
            "TEE multiplier must be nonnegative",
        ));
    }
    // Decimal has a 96-bit mantissa, so this product fits U512 without rounding.
    let numerator = U512::from(price) * U512::from(multiplier.mantissa().unsigned_abs());
    let adjusted = numerator / U512::from(10u128.pow(multiplier.scale()));
    if adjusted > U512::from(U256::MAX) {
        return Err(Status::out_of_range("TEE quote price exceeds uint256"));
    }
    Ok(adjusted.to::<U256>())
}
