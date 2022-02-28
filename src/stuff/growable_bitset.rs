pub struct GrowableBitset {
    state: Vec<usize>,
}

impl GrowableBitset {
    pub fn new() -> Self {
        GrowableBitset { state: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        GrowableBitset {
            state: Vec::with_capacity(cap / usize::BITS as usize),
        }
    }

    #[inline(always)]
    fn get_bucket_shift(pos: usize) -> (usize, usize) {
        (pos / usize::BITS as usize, pos % usize::BITS as usize)
    }

    pub fn set(&mut self, pos: usize) {
        let (bucket_idx, shift) = Self::get_bucket_shift(pos);
        // check if in bounds
        if let Some(bucket) = self.state.get_mut(bucket_idx) {
            *bucket |= 1 << shift;
        } else {
            while bucket_idx >= self.state.len() {
                self.state.push(0);
            }
            self.state[bucket_idx] |= 1 << shift;
        }
    }

    pub fn unset(&mut self, pos: usize) {
        let (bucket_idx, shift) = Self::get_bucket_shift(pos);
        // check if in bounds
        if let Some(bucket) = self.state.get_mut(bucket_idx) {
            *bucket &= !(1 << shift);
        } else {
            while bucket_idx >= self.state.len() {
                self.state.push(0);
            }
            self.state[bucket_idx] &= !(1 << shift);
        }
    }

    pub fn check(&self, pos: usize) -> bool {
        let (bucket_idx, shift) = Self::get_bucket_shift(pos);
        if let Some(bucket) = self.state.get(bucket_idx) {
            return (*bucket & (1 << shift)) != 0;
        } else {
            return false;
        }
    }

    pub fn and(&self, other: &Self) -> Self {
        let new_state = self
            .state
            .iter()
            .zip(other.state.iter())
            .map(|(a, b)| *a & *b)
            .collect();
        GrowableBitset { state: new_state }
    }
}
