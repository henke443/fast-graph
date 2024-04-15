use core::fmt;

#[derive(Debug)]
pub struct LinkedListItem<T: fmt::Debug> {
    pub index: usize,
    pub value: T,
    pub next_index: Option<usize>,
    pub prev_index: Option<usize>,
}

/// A doubly linked list using indexes into a vector instead of pointers for better cache locality than a linked list using pointers and which also solves the ABA problem.
pub struct LinkedList<T: fmt::Debug> {
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub items: Vec<LinkedListItem<T>>
}

impl<T: fmt::Debug> LinkedList<T> {
    /// Create a new empty list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            items: Vec::new(),
        }
    }

    /// Get an item in the list.
    pub fn get(&self, index: usize) -> Option<&LinkedListItem<T>> {
        self.items.get(index).map(|item| item)
    }

    /// Get a mutable reference to an item in the list.
    pub fn get_mut(& mut self, index: usize) -> Option<&mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        item
    }

    /// Get the item after the item with the given index if it exists.
    pub fn next_of(&self, index: usize) -> Option<& LinkedListItem<T>> {
        self.items.get(index).and_then(|item| item.next_index.and_then(|next| self.items.get(next)))
    }

    /// Get the item before the item with the given index if it exists.
    pub fn prev_of(&self, index: usize) -> Option<& LinkedListItem<T>> {
        self.items.get(index).and_then(|item| item.prev_index.and_then(|prev| self.items.get(prev)))
    }

    /// Get a mutable reference to the item after the item with the given index if it exists.
    pub fn next_of_mut(&mut self, index: usize) -> Option<& mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let next = item.and_then(|item| item.prev_index);
        if let Some(next) = next {
            self.items.get_mut(next)
        } else {
            None
        }
    }

    /// Get a mutable reference to the item before the item with the given index if it exists.
    pub fn prev_of_mut(&mut self, index: usize) -> Option<& mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let prev = item.and_then(|item| item.prev_index);
        if let Some(prev) = prev {
            self.items.get_mut(prev)
        } else {
            None
        }
    }

    /// Insert an item after the given index and return the index of the new item.
    pub fn insert_after(&mut self, index: usize, value: T) -> usize {
        let next_index = self.items.get(index).unwrap().next_index;

        let new_index = self.items.len();

        self.items.push(LinkedListItem {
            index: new_index,
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
        new_index
    }

    /// Insert an item before the given index.
    pub fn insert_before(&mut self, index: usize, value: T) -> usize {
        let prev_index = self.items.get(index).unwrap().prev_index;

        let new_index = self.items.len();
        self.items.push(LinkedListItem {
            index: new_index,
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

        new_index
    }


    /// Add an item to the back of the list and return its index.
    pub fn push_back(&mut self, value: T) -> usize {
        let index = self.items.len();
        self.items.push(LinkedListItem {
            index,
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

    /// Push an item to the front of the list.
    pub fn push_front(&mut self, value: T) -> usize {
        let index = self.items.len();
        self.items.push(LinkedListItem {
            index,
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

    /// Remove the last item in the list and return it (if it exists)
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.and_then(|old_tail| {
            let old_tail = self.items.remove(old_tail);

            self.tail = old_tail.prev_index;

            match old_tail.prev_index {
                Some(prev) => {
                    self.items.get_mut(prev).unwrap().next_index = None;
                }
                None => {
                    self.head = None;
                }
            }

            Some(old_tail.value)
        })
    }

    /// Remove the first item in the list and return it (if it exists)
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|old_head| {
            let old_head = self.items.remove(old_head);
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


    pub fn iter_next(&self, start: usize) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.iter_next_index(start).map(move |index| self.items.get(index).unwrap())
    }

    pub fn iter_prev(&self, start: usize) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.iter_prev_index(start).map(move |index| self.items.get(index).unwrap())
    }

    pub fn iter_next_index(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| items.get(*index).and_then(move |item| item.next_index))
    }

    pub fn iter_prev_index(&self, start: usize) -> impl Iterator<Item = usize> + '_  {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| items.get(*index).and_then(move |item| item.prev_index))
    }


    /*
     Splits the list into two at the given index. Returns everything after the given index, including the index.
     This operation should compute in O(n) time.
     */
    // pub fn split_off(&mut self, index: usize) -> Self where T: Clone {
    //     let mut new_list = Self::new();
        
    //     let mut current = index;
    //     while let Some(next) = self.next_of(current).map(|item| item.index) {
    //         let item = self.items.remove(current).unwrap();
    //         new_list.items.insert_with_key(|i| LinkedListItem {
    //             index: i,
    //             value: item.value.clone(),
    //             next_index: None,
    //             prev_index: new_list.tail,
    //         });
    //         current = next;
    //     }

    //     if let Some(prev) = self.prev_of(index).map(|item| item.index) {
    //         self.items.get_mut(prev).unwrap().next_index = None;
    //     } else {
    //         self.head = None;
    //     }

    //     new_list
    // }

    /// Push many items to the back of the list.
    /// 
    /// Returns the indexes of the new items
    pub fn extend<I>(&mut self, values: I) -> Vec<usize> where
        I: IntoIterator<Item = T>,
    {
        let mut indexes = Vec::new();
        for value in values {
            indexes.push(self.push_back(value));
        }
        indexes
    }

    /// Push many items to the front of the list.
    /// 
    /// Returns the indexes of the new items
    pub fn extend_front<I>(&mut self, values: I) -> Vec<usize> where
        I: IntoIterator<Item = T>,
    {
        let mut indexes = Vec::new();
        for value in values {
            indexes.push(self.push_front(value));
        }
        indexes
    }


    /// Get the number of items in the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Remove an item from the list.
    pub fn remove(&mut self, index: usize) -> T {
        let item = self.items.remove(index);

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

    pub fn retain_mut<F>(&mut self, mut f: F) where
        F: FnMut(&T) -> bool,
    {
        let mut current = self.head;
        while let Some(index) = current {
            let item = self.items.get(index).unwrap();
            let next = item.next_index;
            if !f(&item.value) {
                self.remove(index);
            }
            current = next;
        }
    }


    pub fn retain<F>(&self, mut f: F) -> Self where
        F: FnMut(&T) -> bool,
        T: Clone,
        LinkedListItem<T>: Clone
    {
        let mut new_list = Self::new();
        new_list.items = self.items.clone();
        new_list.head = self.head;
        new_list.tail = self.tail;
        new_list.retain_mut(f);
        new_list
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_push_back_fn_next_of_fn_prev_of() {
        let mut list = LinkedList::new();
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
        
    }

    #[test]
    fn test_fn_insert_after_fn_insert_before() {
        // a -> b -> c
        let mut list = LinkedList::new();

        let (a,b,c,d) = {

            let a = list.push_back(1);
            let b = list.push_back(2);
            let c = list.push_back(3);
            let d = list.insert_after(a.clone(), 4);

            // a -> d -> b -> c
            (a,b,c,d)
        };

        
        let prev_b = list.prev_of(b).unwrap();
        let next_d = list.next_of(d).unwrap();
        let next_a = list.next_of(a).unwrap();
        
        assert!(list.prev_of(a).is_none());
        assert_eq!(prev_b.value, 4);
        assert_eq!(next_d.value, 2);
        assert_eq!(next_a.value, 4);
    }

    
    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        let verticies: Vec<usize> = (0..100).map(|i| {
            list.push_back(format!("Node: {}", i.to_string()))
        }).collect();
        
        for n in list.iter_next(verticies[0]) {
            println!("Value: \"{}\"", n.value);
        }

        for n in list.iter_next(verticies[0]) {
            println!("Value: \"{}\"", n.value);
        }
    }

    #[test]
    fn test_popback() {
        let mut list = LinkedList::new();
        let _verticies: Vec<usize> = (0..100).map(|i| {
            list.push_back(format!("Node: {}", i.to_string()))
        }).collect();

        let mut i = 99;
        while let Some(popped) = list.pop_back() {            
            i -= 1;

            //println!("Popped: {:?}", popped);
            let expected = format!("Node: {}", (i).to_string());
            if i >= 0 {
                let last = list.tail.unwrap();
                assert_eq!(list.get(last).unwrap().value, expected);
            } 
        }
    }

}