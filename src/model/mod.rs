pub mod basic;
pub use basic::BasicResponse;

pub mod advanced;
pub use advanced::AdvancedResponse;

pub mod cooldowns;
pub use cooldowns::CooldownResponse;

pub mod battlestats;
pub use battlestats::BattlestatsResponse;

pub mod status;
pub use status::StatusResponse;

pub mod attacks;
pub use attacks::AttacksResponse;
pub use attacks::Attack;

pub mod range;
pub use range::RangeBuilder;
pub use range::Range;
