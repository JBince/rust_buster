use crate::ALL_STATUS_CODES;

pub(super) fn status_codes() -> Vec<u16> {
    ALL_STATUS_CODES
    .iter()
    .map(|code| code.as_u16())
    .collect()
}

