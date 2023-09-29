type BackingType = u32;
const BITS: usize = BackingType::BITS as usize;

pub struct ExpandableBitVec {
    backing: Vec<BackingType>,
}

impl ExpandableBitVec {
    pub fn new(size: usize) -> Self {
        Self {
            backing: vec![0; size / BITS + 1],
        }
    }

    pub fn extend_to_size(&mut self, size: usize) {
        let old_len = self.backing.len();
        let new_len = size / BITS + 1;
        if new_len > old_len {
            self.backing.append(&mut vec![0; new_len - old_len]);
        }
    }

    pub fn index(&mut self, index: usize) -> bool {
        self.extend_to_size(index + 1);
        let vec_ind = index / BITS;
        let Some(el) = self.backing.get(vec_ind) else {
            panic!("Indexing Out of Bounds!");
        };

        let backing_ind = index % BITS;

        el & (1 << backing_ind) > 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.extend_to_size(index + 1);

        let vec_ind = index / BITS;
        let backing_ind = index % BITS;

        if value {
            self.backing[vec_ind] |= 1 << backing_ind;
        } else {
            self.backing[vec_ind] &= BackingType::MAX - (1 << backing_ind);
        }

    }

    pub fn clear(&mut self) {
        for el in &mut self.backing {
            *el = 0;
        }
    }
}