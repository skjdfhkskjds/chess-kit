use crate::state::{History, State};

// HistoryIter is a double-ended iterator over the history
pub struct HistoryIter<'a, S: State> {
    history: &'a History<S>,
    front: usize,
    back: usize,
}

impl<'a, S: State> Iterator for HistoryIter<'a, S> {
    type Item = &'a S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let item = &self.history.states[self.front];
        self.front += 1;
        Some(item)
    }
}

impl<'a, S: State> DoubleEndedIterator for HistoryIter<'a, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            return None;
        }

        let item = &self.history.states[self.back - 1];
        self.back -= 1;
        Some(item)
    }
}

impl<S: State> History<S> {
    pub fn iter(&self) -> HistoryIter<'_, S> {
        HistoryIter {
            history: self,
            front: 0,
            back: self.current,
        }
    }
}
