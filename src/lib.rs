use std::collections::{VecDeque, HashSet};
use std::hash::Hash;

#[derive(Default)]
pub struct UnvisitedIterator<T: Eq + Hash + Clone> {
    visited: HashSet<T>,
    inner: VecDeque<T>,
}

impl<T: Eq + Hash + Clone> UnvisitedIterator<T> {
    pub fn from_value(value: T) -> Self {
        let mut inner = VecDeque::new();
        inner.push_front(value);

        Self {
            visited: HashSet::new(),
            inner,
        }
    }

    pub fn from_iter<Iter: Iterator<Item = T>>(iter: Iter) -> Self {
        Self {
            visited: HashSet::new(),
            inner: iter.collect::<Vec<_>>().into(),
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.inner.push_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value);
    }
}

impl<T: Eq + Hash + Clone> Iterator for UnvisitedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.inner.pop_front() {
            if self.visited.contains(&next) {
                continue;
            }

            self.visited.insert(next.clone());
            return Some(next);
        }

        None
    }
}


pub trait IntoUnvisitedIterator<T: Eq + Hash + Clone> {
    fn skip_visited(self) -> UnvisitedIterator<T>;
}


impl<T: Eq + Hash + Clone, Iter: Iterator<Item = T>> IntoUnvisitedIterator<T> for Iter
{
    fn skip_visited(self) -> UnvisitedIterator<T>
    {
        UnvisitedIterator::from_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_value_initializes_correctly() {
        let iter = UnvisitedIterator::from_value(1);
        assert_eq!(iter.inner.len(), 1);
        assert!(iter.inner.contains(&1));
        assert!(iter.visited.is_empty());
    }

    #[test]
    fn from_iter_initializes_correctly() {
        let iter = UnvisitedIterator::from_iter(vec![1, 2, 3].into_iter());
        assert_eq!(iter.inner.len(), 3);
        assert!(iter.inner.contains(&1));
        assert!(iter.inner.contains(&2));
        assert!(iter.inner.contains(&3));
        assert!(iter.visited.is_empty());
    }

    #[test]
    fn next_returns_unvisited_elements() {
        let mut iter = UnvisitedIterator::from_iter(vec![1, 2, 3].into_iter());
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn next_skips_visited_elements() {
        let mut iter = UnvisitedIterator::from_iter(vec![1, 2, 1, 3].into_iter());
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn push_front_adds_element_to_front() {
        let mut iter = UnvisitedIterator::from_value(1);
        iter.push_front(2);
        assert_eq!(iter.inner.front(), Some(&2));
    }

    #[test]
    fn push_back_adds_element_to_back() {
        let mut iter = UnvisitedIterator::from_value(1);
        iter.push_back(2);
        assert_eq!(iter.inner.back(), Some(&2));
    }
}
