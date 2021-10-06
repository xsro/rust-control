use crate::config::DType;

pub struct ZeroPoleGainModel {
    pub zeros: Vec<DType>,
    pub poles: Vec<DType>,
    pub gain: DType,
    pub sample_time: Option<DType>,
}

impl ZeroPoleGainModel {
    pub fn from(
        zeros: Vec<DType>,
        poles: Vec<DType>,
        gain: DType,
        ts: Option<DType>,
    ) -> ZeroPoleGainModel {
        ZeroPoleGainModel {
            zeros: zeros,
            poles: poles,
            gain: gain,
            sample_time: ts,
        }
    }
}
