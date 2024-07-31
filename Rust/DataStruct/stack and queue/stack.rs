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
pub struct LinkedListStack<T> {
    stack_peek: Option<Rc<RefCell<ListNode<T>>>>,
    stk_size: usize,
}

impl<T: Copy> LinkedListStack<T> {
    pub fn new() -> Self {
        Self {
            stack_peek: None,
            stk_size: 0,
        }
    }

    pub fn size(&self) -> usize {
        return self.stk_size;
    }

    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    pub fn push(&mut self, num: T) {
        let node = ListNode::new(num);
        node.borrow_mut().next = self.stack_peek.take();
        self.stack_peek = Some(node);
        self.stk_size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack_peek.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.stack_peek = Some(new_head);
                }
                None => {
                    self.stack_peek = None;
                }
            }
            self.stk_size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    pub fn peek(&self) -> Option<&Rc<RefCell<ListNode<T>>>> {
        self.stack_peek.as_ref()
    }

    pub fn to_array(&self, head: Option<&Rc<RefCell<ListNode<T>>>>) -> Vec<T> {
        if let Some(node) = head {
            let mut nums = self.to_array(node.borrow_mut().next.as_ref());
            nums.push(node.borrow().val);
            return nums;
        }
        return Vec::new();
    }
}

// Deriver Code
fn main() {
    let mut stack = LinkedListStack::new();

    stack.push(1);
    stack.push(4);
    stack.push(3);
    stack.push(6);
    stack.push(9);

    let peek = stack.peek().unwrap().borrow().val;
    print!("\n栈顶元素 peek = {}", peek);

    let pop = stack.pop().unwrap();
    print!("\n出栈元素 pop = {}", pop);

    let size = stack.size();
    print!("\n栈的长度 size = {}", size);

    let pop = stack.pop().unwrap();
    print!("\n出栈元素 pop = {}", pop);

    let pop = stack.pop().unwrap();
    print!("\n出栈元素 pop = {}", pop);

    let is_empty = stack.is_empty();
    print!("\n is_empty = {}", is_empty);
}
