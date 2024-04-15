use std::marker::PhantomData;

use hashbrown::HashMap;
use slotmap::{new_key_type, KeyData, SlotMap};

new_key_type! {
    pub struct DoublyLinkedListIndex;
}

pub struct DoublyLinkedListItem<T> {
    pub index: DoublyLinkedListIndex,
    pub value: T,
    pub next_index: Option<DoublyLinkedListIndex>,
    pub prev_index: Option<DoublyLinkedListIndex>,
}

// impl<'d, T> DoublyLinkedListItem<'d, T> {
//     pub fn prev(&'d self) -> Option<&'d Self> {
//         self.prev_index.and_then(|index| self.list.get(index))
//     }

//     pub fn next(&'d self) -> Option<&'d Self> {
//         self.next_index.and_then(|index| self.list.get(index))
//     }

//     pub fn prev_mut(&'d self) -> Option<&'d mut Self> {
//         self.prev_index.and_then(|index| self.list.get_mut(index))
//     }

//     pub fn next_mut(&'d self) -> Option<&'d mut Self> {
//         self.next_index.and_then(|index| self.list.get_mut(index))
//     }
// }

/// A doubly linked list using SlotMap for better cache performance than a linked list using pointers and which also solves the ABA problem.
pub struct DoublyLinkedList<T> {
    pub head: Option<DoublyLinkedListIndex>,
    pub tail: Option<DoublyLinkedListIndex>,
    pub items: slotmap::SlotMap<DoublyLinkedListIndex, DoublyLinkedListItem<T>>,
}

struct IterNextMut<'a, T> {
    list: &'a mut DoublyLinkedList<T>,
    current: Option<DoublyLinkedListIndex>,
}

impl<'a, T> IterNextMut<'a, T> {
    fn next(&mut self) -> Option<& mut DoublyLinkedListItem<T>> {
        let current = self.current?;
        let item = self.list.get(current);
        if let Some(item) = item {
            self.current = item.next_index
        }

        if let Some(current) = self.current {
            self.list.get_mut(current)
        } else {
            None
        }
    }
}

struct IterPrevMut<'a, T> {
    list_ref: &'a mut SlotMap<DoublyLinkedListIndex, DoublyLinkedListItem<T>>,
    current: Option<DoublyLinkedListIndex>,
}

impl<'a, T> Iterator for IterPrevMut<'a, T> {
    type Item = DoublyLinkedListIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let item = self.list_ref.get_mut(current);
        self.current = item.unwrap().prev_index;
        item.and_then(|item| Some(item.index))
    }
}



impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            items: slotmap::SlotMap::with_key(),
        }
    }

    pub fn get(&self, index: DoublyLinkedListIndex) -> Option<&DoublyLinkedListItem<T>> {
        self.items.get(index).map(|item| item)
    }

    pub fn get_mut(& mut self, index: DoublyLinkedListIndex) -> Option<& mut DoublyLinkedListItem<T>> {
        let mut item = self.items.get_mut(index);
        item
    }

    pub fn next_of(&self, index: DoublyLinkedListIndex) -> Option<& DoublyLinkedListItem<T>> {
        self.items.get(index).and_then(|item| item.next_index.and_then(|next| self.items.get(next)))
    }

    pub fn prev_of(&self, index: DoublyLinkedListIndex) -> Option<& DoublyLinkedListItem<T>> {
        self.items.get(index).and_then(|item| item.prev_index.and_then(|prev| self.items.get(prev)))
    }

    pub fn next_of_mut(&mut self, index: DoublyLinkedListIndex) -> Option<& mut DoublyLinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let next = item.and_then(|item| item.prev_index);
        if let Some(next) = next {
            self.items.get_mut(next)
        } else {
            None
        }
    }

    pub fn prev_of_mut(&mut self, index: DoublyLinkedListIndex) -> Option<& mut DoublyLinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let prev = item.and_then(|item| item.prev_index);
        if let Some(prev) = prev {
            self.items.get_mut(prev)
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, index: DoublyLinkedListIndex, value: T) -> & DoublyLinkedListItem<T> {
        let next_index = self.items.get(index).unwrap().next_index;

        let new_index = self.items.insert_with_key(|i| DoublyLinkedListItem {
            index: i,
            value,
            next_index: next_index,
            prev_index: Some(index),
        });

        let items = &mut self.items;


        if let Some(next) = next_index {
            // If the element we insert after has a next element, we need to update the next element's `prev` to point to the new element.
            items.get_mut(next).unwrap().prev_index = Some(new_index);
        } else {
            // If the element we insert after does not have a next element, we need to update the tail to point to the new element.
            self.tail = Some(new_index);
        }

        let item = items.get_mut(index).unwrap();
        // Update the element we insert after to point its `prev` to the new element.
        item.next_index = Some(new_index);

        // Return the new element
        self.items.get_mut(new_index).unwrap()
    }

    pub fn insert_before(&mut self, index: DoublyLinkedListIndex, value: T) -> &DoublyLinkedListItem<T> {
        let prev_index = self.items.get(index).unwrap().prev_index;

        let new_index = self.items.insert_with_key(|i| DoublyLinkedListItem {
            index: i,
            value,
            next_index: Some(index),
            prev_index: prev_index,
        });

        let items = &mut self.items;


        if let Some(prev) = prev_index {
            // If the element we insert before has a previous element, we need to update the previous element's `next` to point to the new element.
            items.get_mut(prev).unwrap().next_index = Some(new_index);
        } else {
            // If the element we insert before does not have a previous element, we need to update the head to point to the new element.
            self.head = Some(new_index);
        }

        let item = items.get_mut(index).unwrap();
        // Update the element we insert before to point its `prev` to the new element.
        item.prev_index = Some(new_index);

        self.items.get_mut(new_index).unwrap()
    }

    pub fn push_back(&mut self, value: T) -> DoublyLinkedListIndex {
        let index = self.items.insert_with_key(|i| DoublyLinkedListItem {
            index: i,
            value,
            next_index: None,
            prev_index: self.tail,
        });

        
        match self.tail {
            Some(tail) => {
                self.items.get_mut(tail).unwrap().next_index = Some(index);
            }
            None => {
                self.head = Some(index);
            }
        }

        self.tail = Some(index);

        index
    }

    pub fn push_front(&mut self, value: T) -> DoublyLinkedListIndex {
        let index = self.items.insert_with_key(|i| DoublyLinkedListItem {
            index: i,
            value,
            next_index: self.head,
            prev_index: None,
        });

        match self.head {
            Some(head) => {
                self.items.get_mut(head).unwrap().prev_index = Some(index);
            }
            None => {
                self.tail = Some(index);
            }
        }

        self.head = Some(index);

        index
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|old_tail| {
            let old_tail = self.items.remove(old_tail).unwrap();
            self.tail = old_tail.prev_index;

            match old_tail.prev_index {
                Some(prev) => {
                    self.items.get_mut(prev).unwrap().next_index = None;
                }
                None => {
                    self.head = None;
                }
            }

            old_tail.value
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|old_head| {
            let old_head = self.items.remove(old_head).unwrap();
            self.head = old_head.next_index;

            match old_head.next_index {
                Some(next) => {
                    self.items.get_mut(next).unwrap().prev_index = None;
                }
                None => {
                    self.tail = None;
                }
            }

            old_head.value
        })
    }


    pub fn iter_next(&self, start: DoublyLinkedListIndex) -> impl Iterator<Item = &DoublyLinkedListItem<T>> {
        self.iter_next_index(start).map(move |index| self.items.get(index).unwrap())
    }

    pub fn iter_prev(&self, start: DoublyLinkedListIndex) -> impl Iterator<Item = &DoublyLinkedListItem<T>> {
        self.iter_prev_index(start).map(move |index| self.items.get(index).unwrap())
    }

    pub fn iter_next_index(&self, start: DoublyLinkedListIndex) -> impl Iterator<Item = DoublyLinkedListIndex> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| items.get(*index).and_then(move |item| item.next_index))
    }

    pub fn iter_prev_index(&self, start: DoublyLinkedListIndex) -> impl Iterator<Item = DoublyLinkedListIndex> + '_  {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| items.get(*index).and_then(move |item| item.prev_index))
    }

    pub fn iter_next_mut(&mut self, start: DoublyLinkedListIndex) -> IterNextMut<T> {
        let iter = IterNextMut {
            list: self,
            current: Some(start),
        };

        iter
    }

    /// Extend this list with another list, adding the new items to the back of this list.
    /// 
    /// The other list will be empty after this operation.
    /// 
    /// Returns the indexes of the new items in this list, which will not be the same as the indexes in the source list.
    pub fn extend_back(&mut self, other: &mut Self) -> Vec<DoublyLinkedListIndex> {
        if let Some(tail) = self.tail {
            if let Some(head) = other.head {
                self.items.get_mut(tail).unwrap().next_index = Some(head);
                other.items.get_mut(head).unwrap().prev_index = Some(tail);
            }
        } else {
            self.head = other.head;
        }

        self.tail = other.tail;

        let mut new_indexes = Vec::new();
        let mut index_mapping = HashMap::new();
        let mut other_items = other.items.drain();
        let first_item = other_items.next().unwrap();
        let first_item_index = self.push_back(first_item.1.value);
        for (index, item) in other_items {
            let new_index = self.push_back(item.value);
            index_mapping.insert(index, new_index);
        }
        
        let mut current_item = first_item_index;
        

        new_indexes
    }

    /// Push many items to the back of the list.
    /// 
    /// Returns the indexes of the new items
    pub fn push_back_many(&mut self, values: Vec<T>) -> Vec<DoublyLinkedListIndex> {
        let mut indexes = Vec::with_capacity(values.len());
        for value in values {
            indexes.push(self.push_back(value));
        }
        indexes
    }

    /// Push many items to the front of the list.
    /// 
    /// Returns the indexes of the new items
    pub fn push_front_many(&mut self, values: Vec<T>) -> Vec<DoublyLinkedListIndex> {
        let mut indexes = Vec::with_capacity(values.len());
        for value in values {
            indexes.push(self.push_front(value));
        }
        indexes
    }

    /// Remove an item from the list.
    pub fn remove(&mut self, index: DoublyLinkedListIndex) -> T {
        let item = self.items.remove(index).unwrap();

        if let Some(prev) = item.prev_index {
            self.items.get_mut(prev).unwrap().next_index = item.next_index;
        } else {
            self.head = item.next_index;
        }

        if let Some(next) = item.next_index {
            self.items
                .get_mut(next)
                .unwrap()
                .prev_index = item.prev_index;
        } else {
            self.tail = item.prev_index;
        }

        item.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_push_back_fn_next_of_fn_prev_of() {
        let mut list = DoublyLinkedList::new();
        let a = list.push_back(1);
        let b = list.push_back(2);
        let c = list.push_back(3);


        assert!(list.prev_of(a).is_none());
        assert_eq!(list.get(a).unwrap().value, 1);
        assert_eq!(list.next_of(a).unwrap().value, 2);

        assert_eq!(list.prev_of(b).unwrap().value, 1);
        assert_eq!(list.get(b).unwrap().value, 2);
        assert_eq!(list.next_of(b).unwrap().value, 3);

        assert_eq!(list.prev_of(c).unwrap().value, 2);
        assert_eq!(list.get(c).unwrap().value, 3);
        assert!(list.next_of(c).is_none());


        // A bit paranoid test but let's see so the indexes in the items are correct also
        let a1 = list.get(a).unwrap();
        let b1 = list.get(b).unwrap();
        let c1 = list.get(c).unwrap();
        
        assert!(list.prev_of(a1.index).is_none());
        assert_eq!(a1.value, 1);
        assert_eq!(list.next_of(a1.index).unwrap().value, 2);

        assert_eq!(list.prev_of(b1.index).unwrap().value, 1);
        assert_eq!(b1.value, 2);
        assert_eq!(list.next_of(b1.index).unwrap().value, 3);

        assert_eq!(list.prev_of(c1.index).unwrap().value, 2);
        assert_eq!(c1.value, 3);
        assert!(list.next_of(c1.index).is_none());
        
    }

    #[test]
    fn test_fn_insert_after_fn_insert_before() {
        // a -> b -> c
        let mut list = DoublyLinkedList::new();
        let mut list_items = &mut list.items;

        let (a,b,c,d) = {
            let a = list.push_back(1);
            let b = list.push_back(2);
            let c = list.push_back(3);
    
            // a -> d -> b -> c
            let d = { list.insert_after(a, 4) };
            (a,b,c,d)
        };
        
        let prev_b = list.prev_of(b).unwrap();
        let next_d = list.next_of(d.index).unwrap();
        
        assert_eq!(prev_b.value, 4);
        assert_eq!(next_d.value, 2);
    }

    

}