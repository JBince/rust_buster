use crate::DEFAULT_STATUS_CODES;

pub(super) fn status_codes() -> Vec<u16> {
    DEFAULT_STATUS_CODES
    .iter()
    .map(|code| code.as_u16())
    .collect()
}
