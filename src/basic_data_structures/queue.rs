#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    //Constructor
    fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    //Check empty
    fn is_empty(&self) -> bool {
        0 == self.len()
    }

    //Check full
    fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    //Check length
    fn len(&self) -> usize {
        self.data.len()
    }

    //Clear Queue
    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    //Insert element
    fn enqueue(&mut self, val: T) -> Result<(), &str> {
        if self.len() == self.cap {
            return Err("No space available");
        }
        self.data.insert(0, val);
        Ok(())
    }

    //Pop out values
    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    //Peek Queue
    fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    // Implementation of iterations for a queue
    // into_iter(): queue modified and became a iterator
    // iter(): queue unmodified and get a immutable iterator
    // iter_mut(): queue unmodified and get a mutable iterator
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// Implementation of 3 iterations
struct IntoIter<T>(Queue<T>);
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

struct Iter<'a, T: 'a> {
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

struct IterMut<'a, T: 'a> {
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
    fn test_create_queue() {
        let mut queue: Queue<i32> = Queue::new(5);
        assert_eq!(queue.cap, 5);
        assert_eq!(queue.data.len(), 0);
    }

    #[test]
    fn test_check_empty_queue() {
        let queue: Queue<i32> = Queue {
            cap: 5,
            data: vec![],
        };
        assert!(queue.is_empty());
    }

    #[test]
    fn test_clear_queue() {
        let mut queue: Queue<i32> = Queue {
            cap: 3,
            data: vec![1, 2, 3],
        };
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue() {
        let mut queue_a: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };
        queue_a.enqueue(4).expect("is not at full cap");
        assert_eq!(queue_a.data.len(), 4);

        let mut queue_b: Queue<i32> = Queue {
            cap: 3,
            data: vec![1, 2, 3],
        };
        match queue_b.enqueue(4) {
            Ok(..) => (),
            Err(e) => eprintln!("{}", e),
        }
        assert_eq!(queue_b.data.len(), 3);
    }

    #[test]
    fn test_dequeue() {
        let mut queue_a: Queue<i32> = Queue {
            cap: 3,
            data: vec![1, 2, 3],
        };
        queue_a.dequeue();
        assert_eq!(queue_a.data, vec![2, 3]);

        let mut queue_b: Queue<i32> = Queue {
            cap: 5,
            data: vec![],
        };
        queue_b.dequeue();
        assert_eq!(queue_b.data, vec![]);
    }

    #[test]
    fn test_peek_queue() {
        let queue_a: Queue<i32> = Queue {
            cap: 3,
            data: vec![1, 2, 3],
        };
        assert_eq!(queue_a.peek(), Some(&1));

        let queue_b: Queue<i32> = Queue {
            cap: 2,
            data: vec![],
        };
        assert_eq!(queue_b.peek(), None);
    }

    #[test]
    fn test_get_len_queue() {
        let queue: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };

        assert_eq!(queue.len(), 3)
    }

    #[test]
    fn test_iter_queue() {
        let mut queue: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = queue.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        //Does not panic
        queue.enqueue(1).expect("should not panic");
    }

    #[test]
    fn test_iter_mut_queue() {
        let mut queue: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = queue.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        //Does not panic
        queue.enqueue(1).expect("should not panic");
    }

    #[test]
    fn test_iter_mut_queue_2() {
        let mut queue: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = queue.iter_mut();

        //Modify the original stack from iter_mut
        for x in iter {
            *x *= 2;
        }

        let mut iter2 = queue.iter_mut();

        //Check if original stack has been modified in intended manner
        assert_eq!(iter2.next(), Some(&mut 2));
        assert_eq!(iter2.next(), Some(&mut 4));
        assert_eq!(iter2.next(), Some(&mut 6));
        assert_eq!(iter2.next(), None);

        //Does not panic
        queue.enqueue(1);
    }

    #[test]
    fn test_into_iter_queue() {
        let mut queue: Queue<i32> = Queue {
            cap: 10,
            data: vec![1, 2, 3],
        };

        let mut iter = queue.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        //Panics because into_iter consumes the stack
        //queue.enqueue(1);
    }
}
