#[derive(Clone, Copy)]
pub(crate) struct Entry<T>
where
    T: Copy + Default,
{
    occupied: bool,
    key: u32,
    data: T,
}

impl<T> Entry<T>
where
    T: Copy + Default,
{
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            occupied: false,
            key: 0,
            data: T::default(),
        }
    }

    #[inline]
    pub(crate) const fn is_occupied(&self) -> bool {
        self.occupied
    }

    #[inline]
    pub(crate) const fn key(&self) -> u32 {
        self.key
    }

    #[inline]
    pub(crate) const fn data(&self) -> &T {
        &self.data
    }

    #[inline]
    pub(crate) fn set(&mut self, key: u32, data: T) {
        self.key = key;
        self.data = data;
        self.occupied = true;
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.key = 0;
        self.data = T::default();
        self.occupied = false;
    }
}

impl<T> Default for Entry<T>
where
    T: Copy + Default,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
