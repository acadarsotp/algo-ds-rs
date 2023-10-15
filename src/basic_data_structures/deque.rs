#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    //Constructor
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    //Check empty
    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }

    //Check full
    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    //Check length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    //Clear deque
    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    //Push to the front
    pub fn add_front(&mut self, val: T) -> Result<(), &str> {
        if self.len() == self.cap {
            return Err("No space available");
        }
        self.data.push(val);
        Ok(())
    }

    //Push to the rear
    pub fn add_rear(&mut self, val: T) -> Result<(), &str> {
        if self.len() == self.cap {
            return Err("No space available");
        }
        self.data.insert(0, val);
        Ok(())
    }

    //Pop from the front
    pub fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    //Pop from the rear
    pub fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    //Peek Queue
    pub fn peek_front(&self) -> Option<&T> {
        self.data.last()
    }
    pub fn peek_rear(&self) -> Option<&T> {
        self.data.first()
    }

    // Implementation of iterations for a deque
    // into_iter(): deque modified and became a iterator
    // iter(): deque unmodified and get a immutable iterator
    // iter_mut(): deque unmodified and get a mutable iterator
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// Implementation of 3 iterations
pub struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deque() {
        let deque: Deque<i32> = Deque::new(5);
        assert_eq!(deque.cap, 5);
        assert_eq!(deque.data.len(), 0);
    }

    #[test]
    fn test_check_empty_deque() {
        let deque: Deque<i32> = Deque {
            cap: 5,
            data: vec![],
        };
        assert!(deque.is_empty());
    }

    #[test]
    fn test_check_full_deque() {
        let deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![],
        };
        assert!(!deque_a.is_full());

        let deque_b: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 1, 1, 1, 1],
        };
        assert!(deque_b.is_full());
    }

    #[test]
    fn test_clear_deque() {
        let mut deque: Deque<i32> = Deque {
            cap: 3,
            data: vec![1, 2, 3],
        };
        deque.clear();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_add_front_deque() {
        let mut deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        deque_a.add_front(4).expect("is not at full cap");
        assert_eq!(deque_a.data, vec![1, 2, 3, 4]);

        let mut deque_b: Deque<i32> = Deque {
            cap: 3,
            data: vec![1, 2, 3],
        };
        match deque_b.add_front(4) {
            Ok(..) => (),
            Err(e) => eprintln!("{}", e),
        }
        assert_eq!(deque_b.data.len(), 3);
    }

    #[test]
    fn test_add_rear_deque() {
        let mut deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        deque_a.add_rear(4).expect("is not at full cap");
        assert_eq!(deque_a.data, vec![4, 1, 2, 3]);

        let mut deque_b: Deque<i32> = Deque {
            cap: 3,
            data: vec![1, 2, 3],
        };
        match deque_b.add_rear(4) {
            Ok(..) => (),
            Err(e) => eprintln!("{}", e),
        }
        assert_eq!(deque_b.data.len(), 3);
    }

    #[test]
    fn test_remove_front_deque() {
        let mut deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        deque_a.remove_front();
        assert_eq!(deque_a.data, vec![1, 2]);

        let mut deque_b: Deque<i32> = Deque {
            cap: 5,
            data: vec![],
        };
        deque_b.remove_front();
        assert_eq!(deque_b.data, vec![]);
    }

    #[test]
    fn test_remove_rear_deque() {
        let mut deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        deque_a.remove_rear();
        assert_eq!(deque_a.data, vec![2, 3]);

        let mut deque_b: Deque<i32> = Deque {
            cap: 5,
            data: vec![],
        };
        deque_b.remove_rear();
        assert_eq!(deque_b.data, vec![]);
    }

    #[test]
    fn test_peek_front_deque() {
        let deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        assert_eq!(deque_a.peek_front(), Some(&3));

        let deque_b: Deque<i32> = Deque {
            cap: 2,
            data: vec![],
        };
        assert_eq!(deque_b.peek_front(), None);
    }

    #[test]
    fn test_peek_rear_deque() {
        let deque_a: Deque<i32> = Deque {
            cap: 5,
            data: vec![1, 2, 3],
        };
        assert_eq!(deque_a.peek_rear(), Some(&1));

        let deque_b: Deque<i32> = Deque {
            cap: 2,
            data: vec![],
        };
        assert_eq!(deque_b.peek_rear(), None);
    }

    #[test]
    fn test_get_len_deque() {
        let deque: Deque<i32> = Deque {
            cap: 10,
            data: vec![1, 2, 3],
        };

        assert_eq!(deque.len(), 3)
    }

    #[test]
    fn test_iter_deque() {
        let mut deque: Deque<i32> = Deque {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = deque.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        //Does not panic
        deque.add_front(1).expect("should not panic");
    }

    #[test]
    fn test_iter_mut_deque() {
        let mut deque: Deque<i32> = Deque {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = deque.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        //Does not panic
        deque.add_front(1).expect("should not panic");
    }

    #[test]
    fn test_iter_mut_deque_2() {
        let mut deque: Deque<i32> = Deque {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let iter = deque.iter_mut();

        //Modify the original queue from iter_mut
        for x in iter {
            *x *= 2;
        }

        let mut iter2 = deque.iter_mut();

        //Check if original queue has been modified in intended manner
        assert_eq!(iter2.next(), Some(&mut 2));
        assert_eq!(iter2.next(), Some(&mut 4));
        assert_eq!(iter2.next(), Some(&mut 6));
        assert_eq!(iter2.next(), None);

        //Does not panic
        deque.add_front(1).expect("should not panic");
    }

    #[test]
    fn test_into_iter_deque() {
        let deque: Deque<i32> = Deque {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = deque.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        //Panics because into_iter consumes the queue
        //deque.add_front(1);
    }
}
