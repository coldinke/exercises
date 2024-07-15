use std::vec;
use std::cell::{Cell, RefCell};
use std::rc::Rc;


struct Trunk<'a, 'b> {
    prev: Option<&'a Trunk<'a, 'b>>,
    str: Cell<&'b str>,
}

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub height: i32,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /* 构造方法 */
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            height: 0,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

/* 将列表反序列化为二叉树：递归 */
fn vec_to_tree_dfs(arr: &[Option<i32>], i: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= arr.len() || arr[i].is_none() {
        return None;
    }
    let root = TreeNode::new(arr[i].unwrap());
    root.borrow_mut().left = vec_to_tree_dfs(arr, 2 * i + 1);
    root.borrow_mut().right = vec_to_tree_dfs(arr, 2 * i + 2);
    Some(root)
}

/* 将列表反序列化为二叉树 */
pub fn vec_to_tree(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    vec_to_tree_dfs(&arr, 0)
}

/* 打印二叉树 */
pub fn print_tree(root: &Rc<RefCell<TreeNode>>) {
    _print_tree(Some(root), None, false);
}

/* 打印二叉树 */
fn _print_tree(root: Option<&Rc<RefCell<TreeNode>>>, prev: Option<&Trunk>, is_right: bool) {
    if let Some(node) = root {
        let mut prev_str = "    ";
        let trunk = Trunk { prev, str: Cell::new(prev_str) };
        _print_tree(node.borrow().right.as_ref(), Some(&trunk), true);

        if  prev.is_none() {
            trunk.str.set("———");
        } else if is_right {
            trunk.str.set("/———");
            prev_str = "   |";
        } else {
            trunk.str.set("\\———");
            prev.as_ref().unwrap().str.set(prev_str);
        }

        show_trunks(Some(&trunk));
        println!(" {}", node.borrow().val);
        if let Some(prev) = prev {
            prev.str.set(prev_str);
        }
        trunk.str.set("   |");

        _print_tree(node.borrow().left.as_ref(), Some(&trunk), false);
    }
}

fn show_trunks(trunk: Option<&Trunk>) {
    if let Some(trunk) = trunk {
        show_trunks(trunk.prev);
        print!("{}", trunk.str.get());
    }
}

/* 打印堆 */
pub fn print_heap(heap: Vec<i32>) {
    println!("堆的数组表示：{:?}", heap);
    println!("堆的树状表示：");
    if let Some(root) = vec_to_tree(heap.into_iter().map(|val| Some(val)).collect()) {
        print_tree(&root);
    } 
}

struct MaxHeap {
    max_heap: Vec<i32>,
}

impl MaxHeap {
    fn new(nums: Vec<i32>) -> Self {
        let mut heap = MaxHeap { max_heap: nums };
        for i in (0..=Self::parent(heap.size() - 1)).rev() {
            heap.sift_down(i);
        }
        heap
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.max_heap.swap(i, j);
    }

    fn size(&self) -> usize {
        self.max_heap.len()
    }

    fn is_empty(&self) -> bool {
        self.max_heap.is_empty()
    }

    fn peek(&self) -> Option<i32> {
        self.max_heap.first().copied()
    }

    fn push(&mut self, val: i32) {
        self.max_heap.push(val);
        self.sift_up(self.size() - 1);
    }

    fn sift_up(&mut self, mut i: usize) {
        loop {
            if i == 0 {
                break;
            }
            let p = Self::parent(i);
            if self.max_heap[i] <= self.max_heap[p] {
                break;
            }
            self.swap(i, p);
            i = p;
        }
    }    

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("index out of bounds");
        }
        self.swap(0, self.size() - 1);
        let val = self.max_heap.pop().unwrap();
        self.sift_down(0);
        val
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let (l, r, mut greater) = (Self::left(i), Self::right(i), i);
            if l < self.size() && self.max_heap[l] > self.max_heap[greater] {
                greater = l;
            } 
            if r < self.size() && self.max_heap[r] > self.max_heap[greater] {
                greater = r;
            }
            if greater == i { 
                break;
            }
            self.swap(i, greater);
            i = greater;
        }
    }

    fn print(&self) {
        print_heap(self.max_heap.clone());
    }
}

fn main() {
    /* 初始化大顶堆 */
    let mut max_heap = MaxHeap::new(vec![9, 8, 6, 6, 7, 5, 2, 1, 4, 3, 6, 2]);
    println!("\n输入列表并建堆后");
    max_heap.print();

    /* 获取堆顶元素 */
    let peek = max_heap.peek();
    if let Some(peek) = peek {
        println!("\n堆顶元素为 {}", peek);
    }

    /* 元素入堆 */
    let val = 7;
    max_heap.push(val);
    println!("\n元素 {} 入堆后", val);
    max_heap.print();

    /* 堆顶元素出堆 */
    let peek = max_heap.pop();
    println!("\n堆顶元素 {} 出堆后", peek);
    max_heap.print();

    /* 获取堆大小 */
    let size = max_heap.size();
    println!("\n堆元素数量为 {}", size);

    /* 判断堆是否为空 */
    let is_empty = max_heap.is_empty();
    println!("\n堆是否为空 {}", is_empty);
}