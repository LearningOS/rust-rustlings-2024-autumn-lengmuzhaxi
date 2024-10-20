
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        // Enqueue the new element in q2
        self.q2.enqueue(elem);
        // Move all elements from q1 to q2
        while let Ok(value) = self.q1.dequeue() {
            self.q2.enqueue(value);
        }
        // Swap q1 and q2
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        self.q1.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	
	#[test]
	fn test_queue() {
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Queue is empty")); // Stack is empty
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3)); // LIFO, should return 3
        assert_eq!(s.pop(), Ok(2)); // Should return 2
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false); // Should not be empty
        assert_eq!(s.pop(), Ok(5)); // Should return 5
        assert_eq!(s.pop(), Ok(4)); // Should return 4
        assert_eq!(s.pop(), Ok(1)); // Should return 1
        assert_eq!(s.pop(), Err("Queue is empty")); // Stack is empty
        assert_eq!(s.is_empty(), true); // Should be empty
	}
}
