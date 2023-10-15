#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    //Constructor
    pub fn new() -> Self {
        Stack {
            size: 0,
            data: Vec::new(),
        }
    }

    //Check empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    //Clear the stack
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    //Push to the stack
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    //Pop from the stack
    pub fn pop(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                self.data.pop()
            }
        }
    }

    //Get ref to top value
    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => self.data.last(),
        }
    }

    pub fn get_size(&self) -> &usize {
        &self.size
    }

    // Implementation of iterations for a stack
    // into_iter(): stack modified and became a iterator
    // iter(): stack unmodified and get a immutable iterator
    // iter_mut(): stack unmodified and get a mutable iterator
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
pub struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
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
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_stack() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.size, 0);
        assert_eq!(stack.data.len(), 0);
    }

    #[test]
    fn test_check_empty_stack() {
        let stack: Stack<i32> = Stack {
            size: 0,
            data: vec![],
        };
        assert!(stack.is_empty());
    }

    #[test]
    fn test_clear_stack() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_to_stack() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };
        stack.push(4);
        assert_eq!(stack.size, 4);
        assert_eq!(stack.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pop_from_stack() {
        let mut stack_a: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };
        stack_a.pop();
        assert_eq!(stack_a.size, 2);
        assert_eq!(stack_a.data, vec![1, 2]);

        let mut stack_b: Stack<i32> = Stack::new();
        stack_b.pop();
        assert_eq!(stack_b.size, 0);
        assert_eq!(stack_b.data, vec![]);
    }

    #[test]
    fn test_peek_from_stack() {
        let stack_a: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };
        assert_eq!(stack_a.peek(), Some(&3));

        let stack_b: Stack<i32> = Stack::new();
        assert_eq!(stack_b.peek(), None);
    }

    #[test]
    fn test_get_stack_size() {
        let stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        assert_eq!(stack.get_size(), &3)
    }

    #[test]
    fn test_iter_stack() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        let mut iter = stack.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        //Does not panic
        stack.push(1);
    }

    #[test]
    fn test_iter_mut_stack() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        let mut iter = stack.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);

        //Does not panic
        stack.push(1);
    }

    #[test]
    fn test_iter_mut_stack_2() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        let iter = stack.iter_mut();

        //Modify the original stack from iter_mut
        for x in iter {
            *x *= 2;
        }

        let mut iter2 = stack.iter_mut();

        //Check if original stack has been modified in intended manner
        assert_eq!(iter2.next(), Some(&mut 6));
        assert_eq!(iter2.next(), Some(&mut 4));
        assert_eq!(iter2.next(), Some(&mut 2));
        assert_eq!(iter2.next(), None);

        //Does not panic
        stack.push(1);
    }

    #[test]
    fn test_into_iter_stack() {
        let stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        let mut iter = stack.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        //Panics because into_iter consumes the stack
        //stack.push(1);
    }
}
