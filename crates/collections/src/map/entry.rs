#[derive(Clone, Copy)]
pub struct Entry<T>
where
    T: Copy + Default,
{
    dirty: bool,
    key: u32,
    data: T,
}

impl<T> Entry<T>
where
    T: Copy + Default,
{
    // new creates a new bucket with the given key and data
    //
    // @param: key - key to set the value to
    // @param: data - data to set the value to
    // @return: new bucket
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            dirty: false,
            key: 0,
            data: T::default(),
        }
    }

    // is_dirty checks if the bucket is dirty
    //
    // @return: true if the bucket is dirty, false otherwise
    #[inline(always)]
    pub(crate) const fn is_dirty(&self) -> bool {
        self.dirty
    }

    // key returns the key of the bucket
    //
    // @return: key of the bucket
    #[inline(always)]
    pub(crate) const fn key(&self) -> u32 {
        self.key
    }

    // data returns a reference to the data in the bucket
    //
    // @return: reference to the data in the bucket
    #[inline(always)]
    pub(crate) const fn data(&self) -> &T {
        &self.data
    }

    // set sets the value of the bucket to the given key and data
    //
    // @param: key - key to set the value to
    // @param: data - data to set the value to
    // @return: void
    // @side-effects: modifies the bucket
    #[inline(always)]
    pub(crate) fn set(&mut self, key: u32, data: T) {
        self.key = key;
        self.data = data;
        self.dirty = true;
    }

    // clear clears the bucket to a clean state
    //
    // @return: void
    // @side-effects: modifies the bucket
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        self.key = 0;
        self.data = T::default();
        self.dirty = false;
    }
}

impl<T> Default for Entry<T>
where
    T: Copy + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
