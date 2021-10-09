use crate::config::DType;

pub struct ZeroPoleGain {
    pub zeros: Vec<DType>,
    pub poles: Vec<DType>,
    pub gain: DType,
    pub sample_time: Option<DType>,
}

impl ZeroPoleGain {
    pub fn from(
        zeros: Vec<DType>,
        poles: Vec<DType>,
        gain: DType,
        ts: Option<DType>,
    ) -> ZeroPoleGain {
        ZeroPoleGain {
            zeros: zeros,
            poles: poles,
            gain: gain,
            sample_time: ts,
        }
    }
}
