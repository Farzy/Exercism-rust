use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASECONDS: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(GIGASECONDS, 0)
}
