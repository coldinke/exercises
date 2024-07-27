use std::cell::{Cell, RefCell};
use std::collections::BinaryHeap;
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
        let trunk = Trunk {
            prev,
            str: Cell::new(prev_str),
        };
        _print_tree(node.borrow().right.as_ref(), Some(&trunk), true);

        if prev.is_none() {
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

fn test_push(heap: &mut BinaryHeap<i32>, val: i32, flag: i32) {
    heap.push(flag * val); // 元素入堆
    println!("\n元素 {} 入堆后", val);
    print_heap(heap.iter().map(|&val| flag * val).collect());
}

fn test_pop(heap: &mut BinaryHeap<i32>, flag: i32) {
    let val = heap.pop().unwrap();
    println!("\n堆顶元素 {} 出堆后", flag * val);
    print_heap(heap.iter().map(|&val| flag * val).collect());
}

/* Driver Code */
fn main() {
    /* 初始化堆 */
    // 初始化小顶堆
    #[allow(unused_assignments)]
    let mut min_heap = BinaryHeap::new();
    // Rust 的 BinaryHeap 是大顶堆，当入队时将元素值乘以 -1 将其反转，当出队时将元素值乘以 -1 将其还原
    let min_heap_flag = -1;
    // 初始化大顶堆
    let mut max_heap = BinaryHeap::new();
    let max_heap_flag = 1;

    println!("\n以下测试样例为大顶堆");

    /* 元素入堆 */
    test_push(&mut max_heap, 1, max_heap_flag);
    test_push(&mut max_heap, 3, max_heap_flag);
    test_push(&mut max_heap, 2, max_heap_flag);
    test_push(&mut max_heap, 5, max_heap_flag);
    test_push(&mut max_heap, 4, max_heap_flag);

    /* 获取堆顶元素 */
    let peek = max_heap.peek().unwrap() * max_heap_flag;
    println!("\n堆顶元素为 {}", peek);

    /* 堆顶元素出堆 */
    test_pop(&mut max_heap, max_heap_flag);
    test_pop(&mut max_heap, max_heap_flag);
    test_pop(&mut max_heap, max_heap_flag);
    test_pop(&mut max_heap, max_heap_flag);
    test_pop(&mut max_heap, max_heap_flag);

    /* 获取堆大小 */
    let size = max_heap.len();
    println!("\n堆元素数量为 {}", size);

    /* 判断堆是否为空 */
    let is_empty = max_heap.is_empty();
    println!("\n堆是否为空 {}", is_empty);

    /* 输入列表并建堆 */
    // 时间复杂度为 O(n) ，而非 O(nlogn)
    min_heap = BinaryHeap::from(
        vec![1, 3, 2, 5, 4]
            .into_iter()
            .map(|val| min_heap_flag * val)
            .collect::<Vec<i32>>(),
    );
    println!("\n输入列表并建立小顶堆后");
    print_heap(min_heap.iter().map(|&val| min_heap_flag * val).collect());
}
