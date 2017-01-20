pub struct DisjointSets {
    sets: Vec<i64>,
}

impl DisjointSets {
    pub fn new(size: usize) -> DisjointSets {
        DisjointSets { sets: vec![-1; size] }
    }

    pub fn add_sets(&mut self, count: usize) {
        for _ in 0..count {
            self.sets.push(-1);
        }
    }

    pub fn find_root(&mut self, element: usize) -> Result<usize, String> {
        if element > self.size() {
            return Err("Out of bounds".to_string());
        }

        if self.sets[element] < 0 {
            return Ok(element);
        }

        let parent: usize = self.sets[element] as usize;
        self.sets[element] = try!(self.find_root(parent)) as i64;

        Ok(self.sets[element] as usize)
    }

    pub fn set_union(&mut self, a: usize, b: usize) {
        match (self.find_root(a), self.find_root(b)) {
            (Ok(root_a), Ok(root_b)) => {
                if root_a == root_b {
                    return;
                }

                if self.sets[root_a] <= self.sets[root_b] {
                    self.sets[root_a] += self.sets[root_b];
                    self.sets[root_b] = root_a as i64;
                } else {
                    self.sets[root_b] += self.sets[root_a];
                    self.sets[root_a] = root_b as i64;
                }
            }
            _ => panic!("Unable to get roots in order to merge sets."),
        }
    }

    pub fn size(&self) -> usize {
        self.sets.len()
    }
}
