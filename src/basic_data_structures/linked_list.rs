type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    size: usize,
    head: Link<T>,
}

#[derive(PartialEq, Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T: PartialEq> List<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    fn insert_at(&mut self, index: usize, element: T) -> Result<(), &str> {
        if index == 0 {
            self.push(element);
            return Ok(());
        }
        let mut target = &mut self.head;
        for _ in 0..index {
            if let Some(node) = target {
                target = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        let new_node = Box::new(Node {
            element,
            next: target.take(),
        });
        *target = Some(new_node);
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.element
        })
    }

    fn delete_at(&mut self, index: usize) -> Result<Option<T>, &str> {
        if index == 0 {
            return Ok(self.pop());
        }
        let mut target = &mut self.head;
        for _ in 0..index {
            if let Some(node) = target {
                target = &mut node.next;
            } else {
                return Err("Index out of bounds");
            }
        }
        if let Some(node) = target.take() {
            *target = node.next;
            self.size -= 1;
            Ok(Some(node.element))
        } else {
            Ok(None)
        }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    // Implementation of iteration for the linked list.
    // into_iter: makes the linked list an iterator by consuming it
    // iter: returns an immutable iterator without modifying the linked list
    // iter_mut: returns a mutable iterator without modifying the linked list

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    //Find element using iterator
    fn find(&self, element: T) -> Option<Vec<usize>> {
        let found_indices: Vec<usize> = self
            .iter()
            .enumerate()
            .filter(|&(_, item)| *item == element)
            .map(|(index, _)| index)
            .collect();

        match found_indices.len() {
            0 => None,
            _ => Some(found_indices),
        }
    }
}

//Implementation of 3 iterators
struct IntoIter<T>(List<T>);
impl<T: PartialEq> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // (List<T>) tuple's 0th item
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

// Custom implementation of Drop for the linked list
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list: List<i32> = List::new();

        assert_eq!(list.size, 0);
        assert_eq!(list.head, None);
    }

    #[test]
    fn test_empty_list() {
        let list: List<i32> = List::new();

        assert!(list.is_empty());
    }

    #[test]
    fn test_len_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_peek_list() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek_mut(), None);

        list.push(2);
        list.push(5);

        assert_eq!(list.peek_mut(), Some(&mut 5));
        assert_eq!(list.peek(), Some(&5));
    }

    #[test]
    fn test_push_iter_pop_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);
        list.push(10);

        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&10, &5, &2]);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&5, &2]);
    }

    #[test]
    fn test_insert_at_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);
        list.push(10);

        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&10, &5, &2]);
        let _ = list.insert_at(2, 60);
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&10, &5, &60, &2]);
    }

    #[test]
    fn test_delete_at_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);
        list.push(10);

        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&10, &5, &2]);
        let _ = list.delete_at(2);
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&10, &5]);
    }

    #[test]
    fn test_find_at_list() {
        let mut list_a: List<i32> = List::new();

        let mut counter = 0;
        for _ in 0..=1000 {
            list_a.push(counter);
            counter += 1;
        }

        assert_eq!(list_a.find(563), Some(vec![1000-563]));

        let mut list_b: List<i32> = List::new();
        list_b.push(3);
        list_b.push(12);
        list_b.push(3);
        list_b.push(54);

        assert_eq!(list_b.find(3), Some(vec![1,3]));
    }

    #[test]
    fn test_iter_mut_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);
        list.push(10);

        assert_eq!(
            list.iter_mut().collect::<Vec<_>>(),
            vec![&mut 10, &mut 5, &mut 2]
        );

        for x in list.iter_mut() {
            *x *= 2;
        }

        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&20, &10, &4]);
    }

    #[test]
    fn test_into_iter_list() {
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(5);
        list.push(10);

        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![10, 5, 2]);

        //Panics because list is consumed by iterator
        //list.push(1);
    }
}
