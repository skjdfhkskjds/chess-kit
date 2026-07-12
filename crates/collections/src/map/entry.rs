#[derive(Clone, Copy)]
pub(crate) struct Entry<T>
where
    T: Copy,
{
    key: u32,
    data: T,
}

impl<T> Entry<T>
where
    T: Copy,
{
    #[inline]
    pub(crate) const fn new(key: u32, data: T) -> Self {
        Self { key, data }
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
    }
}
