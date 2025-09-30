use time::{PrimitiveDateTime as DateTime, Duration};

pub fn after(start: DateTime) -> DateTime {
    return start + Duration::seconds(1000000000);
}
