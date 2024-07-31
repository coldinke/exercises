use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Rc<RefCell<ListNode<T>>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

#[allow(dead_code)]
pub struct LinkedListQueue<T> {
    front: Option<Rc<RefCell<ListNode<T>>>>,
    rear: Option<Rc<RefCell<ListNode<T>>>>,
    que_size: usize,
}

impl<T: Copy> LinkedListQueue<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            que_size: 0,
        }
    }

    pub fn size(&self) -> usize {
        return self.que_size;
    }

    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    pub fn push(&mut self, num: T) {
        let new_rear = ListNode::new(num);
        match self.rear.take() {
            Some(old_rear) => {
                old_rear.borrow_mut().next = Some(new_rear.clone());
                self.rear = Some(new_rear);
            }
            None => {
                self.front = Some(new_rear.clone());
                self.rear = Some(new_rear);
            }
        }
        self.que_size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    self.front = Some(new_front);
                }
                None => {
                    self.rear.take();
                }
            }
            self.que_size -= 1;
            Rc::try_unwrap(old_front).ok().unwrap().into_inner().val
        })
    }

    pub fn peek(&self) -> Option<&Rc<RefCell<ListNode<T>>>> {
        self.front.as_ref()
    }

    pub fn to_array(&self, head: Option<&Rc<RefCell<ListNode<T>>>>) -> Vec<T> {
        if let Some(node) = head {
            let mut nums = self.to_array(node.borrow().next.as_ref());
            nums.insert(0, node.borrow().val);
            return nums;
        }
        return Vec::new();
    }
}

// Deriver Code
fn main() {
    let mut queue = LinkedListQueue::new();

    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(5);
    queue.push(3);
    queue.push(9);

    let peek = queue.peek().unwrap().borrow().val;
    print!("\n队首元素 peek = {}", peek);

    let pop = queue.pop().unwrap();
    print!("\n出队元素 pop = {}", pop);

    let peek = queue.peek().unwrap().borrow().val;
    print!("\n队首元素 peek = {}", peek);

    let pop = queue.pop().unwrap();
    print!("\n出队元素 pop = {}", pop);

    let peek = queue.peek().unwrap().borrow().val;
    print!("\n队首元素 peek = {}", peek);

    let size = queue.size();
    print!("\n队列长度 size = {}", size);

    let is_empty = queue.is_empty();
    print!("\n队列是否为空 = {}", is_empty);
}
