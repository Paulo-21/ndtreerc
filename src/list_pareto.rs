#[derive(Debug, Clone)]
pub struct ParetoList {
    pub list: Vec<[i32; 8]>,
}
impl ParetoList {
    pub fn insert(&mut self, pt: [i32; 8]) -> bool {
        let mut to_insert = true;
        let mut to_remove = Vec::new();
        for (idx, el) in self.list.iter().enumerate() {
            let mut has_low = false;
            let mut has_up = false;
            for (nondp, currp) in el.iter().zip(pt) {
                has_low |= currp < *nondp;
                has_up |= currp > *nondp;
            }
            if !has_low && has_up {
                to_remove.push(idx);
            } else if has_low && !has_up {
                to_insert = false;
                break;
            }
            //to_insert &= !has_low || has_up; // 200ms win
        }
        if !to_remove.is_empty() {
            for (k, idx) in to_remove.iter().enumerate() {
                self.list.remove(idx - k);
            }
        }
        if to_insert {
            self.list.push(pt);
        }
        to_insert
    }
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
}
