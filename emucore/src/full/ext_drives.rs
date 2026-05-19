use crate::common::line::{multi::BusDriveState, single::DriveState};

pub struct ExtDrives {
    pub a: BusDriveState<13>,
    pub db: BusDriveState<8>,
    pub inp1: BusDriveState<7>,
    pub inp2: BusDriveState<7>,
    pub rdiff: DriveState,
    pub ldiff: DriveState,
    pub col: DriveState,
    pub sel: DriveState,
    pub res: DriveState,
}
