use strum::{Display, EnumString, FromRepr};

#[repr(u8)]
#[derive(Display, Debug, FromRepr, EnumString, Clone, Copy, Eq, PartialEq)]
pub enum LaunchStages {
    Stopped,
    Stopping,
    Started,
    Starting,
}
