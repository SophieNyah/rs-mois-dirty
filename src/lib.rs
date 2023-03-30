pub struct Dirty<T> {
    dirty: bool,
    value: T,
}

impl<T: Default> Default for Dirty<T> {
    fn default() -> Self { Self{dirty: false, value: Default::default() } }
}

impl<T: PartialEq> PartialEq for Dirty<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.dirty == other.dirty) && (self.value == other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        (self.dirty != other.dirty) || (self.value != self.value)
    }
}

impl<T> Dirty<T> {
    pub fn update_if_dirty<F>(&mut self, f: F) -> &T
        where F:FnOnce() -> T{
        if !self.dirty {
            &self.value
        } else {
            self.dirty = false;
            self.value = f();
            &self.value
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn update(&mut self, v: T) {
        self.dirty = true;
        self.value = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_default() {
        let d: Dirty<usize> = Default::default();
        assert_eq!(d.dirty, false);
        assert_eq!(d.value, 0);
    }

    #[test]
    fn dirty_update() {
        let mut d: Dirty<usize> = Default::default();
        d.update(15);
        assert_eq!(d.dirty, true);
        assert_eq!(d.value, 15);
    }

    #[test]
    fn dirty_update_if_dirty() {
        let mut d: Dirty<usize> = Default::default();
        d.update_if_dirty(|| 9);
        assert_eq!(d.dirty, false);
        assert_eq!(d.value, 0);

        d.update(7);
        d.update_if_dirty(|| 20);
        assert_eq!(d.dirty, false);
        assert_eq!(d.value, 20);
    }

    #[test]
    fn dirty_set_dirty() {
        let mut d: Dirty<usize> = Default::default();
        d.set_dirty();
        assert_eq!(d.dirty, true);
    }

    #[test]
    fn dirty_clear_dirty () {
        let mut d: Dirty<usize> = Default::default();
        d.update(8);
        d.clear_dirty();
        assert_eq!(d.dirty, false);
    }

}
