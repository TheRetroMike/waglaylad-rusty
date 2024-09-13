use crate::error::Error;
use crate::result::Result;
use waglayla_consensus_core::constants::SOMPI_PER_WAGLAYLA;
use std::fmt::Display;

pub fn try_parse_required_nonzero_waglayla_as_sompi_u64<S: ToString + Display>(waglayla_amount: Option<S>) -> Result<u64> {
    if let Some(waglayla_amount) = waglayla_amount {
        let sompi_amount = waglayla_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Waglayla amount is not valid: '{waglayla_amount}'")))?
            * SOMPI_PER_WAGLAYLA as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Waglayla amount is not valid: '{waglayla_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required waglayla amount must not be a zero: '{waglayla_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Waglayla amount"))
    }
}

pub fn try_parse_required_waglayla_as_sompi_u64<S: ToString + Display>(waglayla_amount: Option<S>) -> Result<u64> {
    if let Some(waglayla_amount) = waglayla_amount {
        let sompi_amount = waglayla_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Waglayla amount is not valid: '{waglayla_amount}'")))?
            * SOMPI_PER_WAGLAYLA as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Waglayla amount is not valid: '{waglayla_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Waglayla amount"))
    }
}

pub fn try_parse_optional_waglayla_as_sompi_i64<S: ToString + Display>(waglayla_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(waglayla_amount) = waglayla_amount {
        let sompi_amount = waglayla_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Waglayla amount is not valid: '{waglayla_amount}'")))?
            * SOMPI_PER_WAGLAYLA as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Waglayla amount is not valid: '{waglayla_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
