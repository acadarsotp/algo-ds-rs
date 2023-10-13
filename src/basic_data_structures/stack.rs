
#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    //Constructor
    fn new() -> Self {
        Stack {
            size: 0,
            data: Vec::new(),
        }
    }

    //Check empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    //Clear the stack
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    //Push to the stack
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    //Pop from the stack
    fn pop(&mut self) {
        match self.size {
            0 => (),
            _ => {
                self.data.pop();
                self.size -= 1;
            }
        }
    }

    //Get ref to top value
    fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => self.data.last(),
        }
    }

    //Get mut ref to top value
    fn peek_mut(&mut self) -> Option<&mut T> {
        match self.size {
            0 => None,
            _ => self.data.last_mut(),
        }
    }

    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_size(&self) -> &usize {
        &self.size
    }
}

//Implement iterator trait

impl<T> Iterator for Stack<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                self.data.pop()
            }
        }
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
    fn test_peek_mut_from_stack() {
        let mut stack_a: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };
        assert_eq!(stack_a.peek_mut(), Some(&mut 3));

        let mut stack_b: Stack<i32> = Stack::new();
        assert_eq!(stack_b.peek_mut(), None);
    }

    #[test]
    fn test_iter_from_stack() {
        let mut stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        assert_eq!(stack.next(), Some(3));
        assert_eq!(stack.next(), Some(2));
        assert_eq!(stack.next(), Some(1));
        assert_eq!(stack.next(), None);
    }

    #[test]
    fn test_stack_data() {
        let stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        let v = stack.get_data();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_get_stack_size() {
        let stack: Stack<i32> = Stack {
            size: 3,
            data: vec![1, 2, 3],
        };

        assert_eq!(stack.get_size(), &3)
    }
}
