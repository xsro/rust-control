pub struct ZeroPoleGain<T, U> {
    pub zeros: Vec<T>,
    pub poles: Vec<T>,
    pub gain: T,
    pub sample_time: Option<U>,
}

impl<T, U> ZeroPoleGain<T, U> {
    pub fn from(zeros: Vec<T>, poles: Vec<T>, gain: T, ts: Option<U>) -> ZeroPoleGain<T, U> {
        ZeroPoleGain {
            zeros: zeros,
            poles: poles,
            gain: gain,
            sample_time: ts,
        }
    }
}
