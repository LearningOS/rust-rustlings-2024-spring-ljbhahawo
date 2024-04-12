/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


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
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
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

pub struct MyStack<T>
{
	// true为q1，false为q2
    
    main_queue:bool,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            // 默认主队列为q1            
            main_queue:true,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
            
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.main_queue {
            self.q1.enqueue(elem);
        }else {
            self.q2.enqueue(elem)
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // pop的时候，将目前的主队列的最后一个元素前的所有元素，放入另一个队列
        if self.q1.is_empty() && self.q2.is_empty(){
            Err("Stack is empty")
        }else {
            if self.main_queue{
                while self.q1.size() >1{
                    self.q2.enqueue(self.q1.dequeue().unwrap());
                }
                self.main_queue = false;
                self.q1.dequeue()
            }else {
                while self.q2.size() >1{
                    self.q1.enqueue(self.q2.dequeue().unwrap());
                }
                self.main_queue = true;
                self.q2.dequeue()
            }
        }
		
    }
    pub fn is_empty(&self) -> bool {
		self.q1.is_empty() && self.q2.is_empty()
        
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}