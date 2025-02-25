use slotmap::{DefaultKey, SlotMap};

#[derive(Debug)]
pub struct SlotListNode<T> {
    value: T,
    prev: Option<DefaultKey>,
    next: Option<DefaultKey>,
}

#[derive(Debug)]
pub struct SlotList<T> {
    inner: SlotMap<DefaultKey, SlotListNode<T>>,
    head: DefaultKey,
    tail: DefaultKey,
}

impl<T> SlotList<T> {
    pub fn with_capacity(initial: T, capacity: usize) -> Self {
        let mut nodes = SlotMap::with_capacity(capacity);

        let first_key = nodes.insert(SlotListNode {
            value: initial,
            prev: None,
            next: None,
        });

        Self {
            inner: nodes,
            head: first_key,
            tail: first_key,
        }
    }

    pub fn head(&self) -> DefaultKey {
        self.head
    }

    pub fn tail(&self) -> DefaultKey {
        self.tail
    }

    pub fn insert_after(&mut self, idx: DefaultKey, value: T) -> DefaultKey {
        let next = self.inner[idx].next;
        let new_key = self.inner.insert(SlotListNode {
            value,
            prev: Some(idx),
            next,
        });

        if let Some(next_key) = next {
            self.inner[next_key].prev = Some(new_key);
        } else {
            self.tail = new_key;
        }

        self.inner[idx].next = Some(new_key);

        new_key
    }

    pub fn append(&mut self, value: T) -> DefaultKey {
        let new_key = self.inner.insert(SlotListNode {
            value,
            prev: Some(self.tail),
            next: None,
        });

        self.inner[self.tail].next = Some(new_key);
        self.tail = new_key;

        new_key
    }

    pub fn next(&self, idx: DefaultKey) -> Option<&SlotListNode<T>> {
        self.inner[idx].next.map(|next| &self.inner[next])
    }

    pub fn next_key(&self, idx: DefaultKey) -> Option<DefaultKey> {
        self.inner[idx].next
    }

    pub fn next_mut(&mut self, idx: DefaultKey) -> Option<&mut SlotListNode<T>> {
        self.inner[idx].next.map(|next| &mut self.inner[next])
    }

    pub fn prev(&self, idx: DefaultKey) -> Option<&SlotListNode<T>> {
        self.inner[idx].prev.map(|prev| &self.inner[prev])
    }

    pub fn prev_key(&self, idx: DefaultKey) -> Option<DefaultKey> {
        self.inner[idx].prev
    }

    pub fn prev_mut(&mut self, idx: DefaultKey) -> Option<&mut SlotListNode<T>> {
        self.inner[idx].prev.map(|prev| &mut self.inner[prev])
    }

    pub fn iter<'a>(&'a self) -> SlotListIter<'a, T> {
        SlotListIter {
            list: &self.inner,
            curr_head: Some(self.head),
            curr_tail: Some(self.tail),
        }
    }

    pub fn iter_custom(
        &self,
        head: Option<DefaultKey>,
        tail: Option<DefaultKey>,
    ) -> SlotListIter<'_, T> {
        SlotListIter {
            list: &self.inner,
            curr_head: head,
            curr_tail: tail,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove(&mut self, idx: DefaultKey) {
        assert!(idx != self.head);

        let removed = self.inner.remove(idx).unwrap();

        let prev = removed.prev.expect("Head is never removed");
        self.inner[prev].next = removed.next;

        if let Some(next) = removed.next {
            self.inner[next].prev = Some(prev);
        }

        if self.tail == idx {
            self.tail = prev;
        }
    }
}

impl<T: Clone> std::clone::Clone for SlotList<T> {
    fn clone(&self) -> Self {
        let mut values = self.iter().map(|key| self.inner[key].value.clone());
        let initial = values.next().expect("One element always guaranteed");

        let mut new_list = SlotList::with_capacity(initial, self.len());

        for value in values {
            new_list.append(value);
        }

        new_list
    }
}

impl<T: PartialEq> std::cmp::PartialEq for SlotList<T> {
    fn eq(&self, other: &Self) -> bool {
        fn to_value_tuple<'a, T>(
            key: DefaultKey,
            list: &'a SlotList<T>,
        ) -> (&'a T, Option<&'a T>, Option<&'a T>) {
            let curr = &list.inner[key];
            let prev = curr.prev.map(|prev| &list.inner[prev].value);
            let next = curr.next.map(|next| &list.inner[next].value);

            (&curr.value, prev, next)
        }

        self.iter()
            .map(|key| to_value_tuple(key, self))
            .eq(other.iter().map(|key| to_value_tuple(key, other)))
    }
}

impl<T> std::ops::Index<DefaultKey> for SlotList<T> {
    type Output = T;

    fn index(&self, index: DefaultKey) -> &Self::Output {
        &self.inner[index].value
    }
}

impl<T> std::ops::IndexMut<DefaultKey> for SlotList<T> {
    fn index_mut(&mut self, index: DefaultKey) -> &mut Self::Output {
        &mut self.inner[index].value
    }
}

pub struct SlotListIter<'a, T> {
    list: &'a SlotMap<DefaultKey, SlotListNode<T>>,
    curr_head: Option<DefaultKey>,
    curr_tail: Option<DefaultKey>,
}

impl<T> Iterator for SlotListIter<'_, T> {
    type Item = DefaultKey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_head == self.curr_tail {
            let item = self.curr_head;

            self.curr_head = None;
            self.curr_tail = None;

            item
        } else {
            let item = self
                .curr_head
                .expect("None case should caught in the If clause");
            self.curr_head = self.list[item].next;

            Some(item)
        }
    }
}

impl<T> DoubleEndedIterator for SlotListIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.curr_tail == self.curr_head {
            let item = self.curr_tail;

            self.curr_tail = None;
            self.curr_head = None;

            item
        } else {
            let item = self
                .curr_tail
                .expect("None case should caught in the If clause");
            self.curr_tail = self.list[item].prev;

            Some(item)
        }
    }
}

#[cfg(test)]
mod slot_list_tests {
    use super::*;

    fn to_values(list: &SlotList<i32>) -> Vec<i32> {
        let forward = list
            .iter()
            .map(|key| list.inner[key].value)
            .collect::<Vec<_>>();

        let mut backward = list
            .iter()
            .rev()
            .map(|key| list.inner[key].value)
            .collect::<Vec<_>>();

        backward.reverse();
        assert_eq!(forward, backward);

        let clone_list = list.clone();

        assert_eq!(clone_list, *list);

        forward
    }

    #[test]
    fn test_forward_iteration_matches_vector() {
        let seed = vec![1, 2, 3, 4, 5];
        let mut list: SlotList<i32> = SlotList::with_capacity(seed[0], 10);

        for &val in seed.iter().skip(1) {
            list.append(val);
        }

        let actual: Vec<i32> = list.iter().map(|key| list.inner[key].value).collect();
        assert_eq!(actual, seed);

        let reversed_seed: Vec<i32> = seed.iter().rev().cloned().collect();
        let actual_reversed: Vec<i32> =
            list.iter().rev().map(|key| list.inner[key].value).collect();
        assert_eq!(actual_reversed, reversed_seed);
    }

    #[test]
    fn test_custom_iteration_matches_vector_slice() {
        let seed = vec![1, 2, 3, 4, 5, 6];
        let mut list: SlotList<i32> = SlotList::with_capacity(seed[0], 10);
        let mut keys: Vec<DefaultKey> = vec![list.head];
        for &val in seed.iter().skip(1) {
            keys.push(list.append(val));
        }

        let start_index = 2;
        let end_index = 5;
        let custom_start_key = Some(keys[start_index]);
        let custom_end_key = Some(keys[end_index]);

        let vector_slice: Vec<i32> = seed[start_index..=end_index].to_vec();
        let collected_vector: Vec<i32> = list
            .iter_custom(custom_start_key, custom_end_key)
            .map(|key| list.inner[key].value)
            .collect();
        assert_eq!(collected_vector, vector_slice);

        let reversed_vector_slice: Vec<i32> = vector_slice.iter().rev().cloned().collect();
        let collected_reversed_vector: Vec<i32> = list
            .iter_custom(custom_start_key, custom_end_key)
            .rev()
            .map(|key| list.inner[key].value)
            .collect();
        assert_eq!(collected_reversed_vector, reversed_vector_slice);
    }

    #[test]
    fn test_iter_empty_list() {
        let seed: Vec<i32> = vec![10];
        let list: SlotList<i32> = SlotList::with_capacity(seed[0], 5);

        let collected_vector: Vec<i32> = list.iter().map(|key| list.inner[key].value).collect();
        assert_eq!(collected_vector, seed);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(list.head));
        assert_eq!(iter.next(), None);

        let collected_rev_vector: Vec<i32> =
            list.iter().rev().map(|key| list.inner[key].value).collect();
        let reversed_seed: Vec<i32> = seed.iter().rev().cloned().collect();
        assert_eq!(collected_rev_vector, reversed_seed);

        let mut iter_rev = list.iter();
        assert_eq!(iter_rev.next_back(), Some(list.tail));
        assert_eq!(iter_rev.next_back(), None);
    }

    #[test]
    fn test_iter_interleaved_even_list() {
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 10);
        list.append(2);
        list.append(3);
        list.append(4);

        let mut iter = list.iter();
        let mut actual = Vec::new();

        let to_value = |key: DefaultKey| list.inner[key].value;

        actual.push(iter.next().map(|key| to_value(key))); // 1
        actual.push(iter.next_back().map(|key| to_value(key))); // 4
        actual.push(iter.next().map(|key| to_value(key))); // 2
        actual.push(iter.next_back().map(|key| to_value(key))); // 3
        actual.push(iter.next().map(|key| to_value(key))); // None
        actual.push(iter.next_back().map(|key| to_value(key))); // None

        assert_eq!(actual, vec![Some(1), Some(4), Some(2), Some(3), None, None]);
    }

    #[test]
    fn test_iter_interleaved_odd_list() {
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 10);
        list.append(2);
        list.append(3);

        let mut iter = list.iter();
        let mut actual = Vec::new();

        let to_value = |key: DefaultKey| list.inner[key].value;

        actual.push(iter.next().map(|key| to_value(key))); // 1
        actual.push(iter.next_back().map(|key| to_value(key))); // 3
        actual.push(iter.next().map(|key| to_value(key))); // 2
        actual.push(iter.next_back().map(|key| to_value(key))); // None
        actual.push(iter.next().map(|key| to_value(key))); // None

        assert_eq!(actual, vec![Some(1), Some(3), Some(2), None, None]);
    }

    #[test]
    fn test_iter_interleaved_single_element_list() {
        let list: SlotList<i32> = SlotList::with_capacity(1, 10); // List: [1]
        let mut iter = list.iter();
        let mut actual = Vec::new();

        let to_value = |key: DefaultKey| list.inner[key].value;

        actual.push(iter.next().map(|key| to_value(key))); // Some(1)
        actual.push(iter.next_back().map(|key| to_value(key))); // None
        actual.push(iter.next().map(|key| to_value(key))); // None

        assert_eq!(actual, vec![Some(1), None, None]);
    }

    #[test]
    fn test_insert_after_head() {
        // Single element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        let head_key = list.head;
        list.insert_after(head_key, 2);
        assert_eq!(
            to_values(&list),
            vec![1, 2],
            "Single element list - insert after head failed"
        );

        // Two element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        list.append(2);
        let head_key = list.head;
        list.insert_after(head_key, 3);
        assert_eq!(
            to_values(&list),
            vec![1, 3, 2],
            "Two element list - insert after head failed"
        );

        // Four element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        list.append(2);
        list.append(3);
        list.append(4);
        let head_key = list.head;
        list.insert_after(head_key, 5);
        assert_eq!(
            to_values(&list),
            vec![1, 5, 2, 3, 4],
            "Four element list - insert after head failed"
        );
    }

    #[test]
    fn test_insert_after_tail() {
        // Single element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        let tail_key = list.tail; // Tail is same as head
        list.insert_after(tail_key, 2);
        assert_eq!(
            to_values(&list),
            vec![1, 2],
            "Single element list - insert after tail failed"
        );

        // Two element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        list.append(2);
        let tail_key = list.tail;
        list.insert_after(tail_key, 3);
        assert_eq!(
            to_values(&list),
            vec![1, 2, 3],
            "Two element list - insert after tail failed"
        );

        // Four element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        list.append(2);
        list.append(3);
        list.append(4);
        let tail_key = list.tail;
        list.insert_after(tail_key, 5);
        assert_eq!(
            to_values(&list),
            vec![1, 2, 3, 4, 5],
            "Four element list - insert after tail failed"
        );
    }

    #[test]
    fn test_insert_after_middle() {
        let to_values = |list: &SlotList<i32>| {
            list.iter()
                .map(|key| list.inner[key].value)
                .collect::<Vec<_>>()
        };

        // Two element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        let key1 = list.append(2);
        list.insert_after(key1, 4);
        list.insert_after(key1, 3);
        list.insert_after(list.tail, 5);

        assert_eq!(
            to_values(&list),
            vec![1, 2, 3, 4, 5],
            "Two element list - insert after middle failed"
        );

        // Four element list
        let mut list: SlotList<i32> = SlotList::with_capacity(1, 5);
        let key = list.append(2);
        list.append(3);
        list.append(4);
        list.insert_after(key, 5);
        assert_eq!(
            to_values(&list),
            vec![1, 2, 5, 3, 4],
            "Four element list - insert after middle failed"
        );
    }
}
