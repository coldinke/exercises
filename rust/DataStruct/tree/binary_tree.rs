use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();

            inorder(n.left.clone(), result);

            result.push(n.val);

            inorder(n.right.clone(), result);
        }
    }

    inorder(root, &mut result);

    result
}

fn main() {
    // Initializing nodes
    let n1 = TreeNode::new(1);
    let n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n4 = TreeNode::new(4);
    let n5 = TreeNode::new(5);

    // 这里的 Rc<RefCell<TreeNode>> 提供了内部可变性。
    // Rc 本身是不可变的，但 RefCell 允许我们在运行时借用起内部的可变引用。
    // clone 的操作来自 Rc 类型，而 Rc 是引用计数智能指针，
    // 当我们 clone 时实际上是增加了引用计数，而不是深度复制整个数据结构
    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3);
    n2.borrow_mut().left = Some(n4);
    n2.borrow_mut().right = Some(n5);

    // insert node to tree
    let p = TreeNode::new(0);
    n1.borrow_mut().left = Some(p.clone());
    p.borrow_mut().left = Some(n2.clone());

    let result = inorder_traversal(Some(n1.clone()));

    println!("Inorder traversal result: {:?} ", result);

    // delete node from tree
    n1.borrow_mut().left = Some(n2);

    let result = inorder_traversal(Some(n1.clone()));
    println!("Inorder traversal result: {:?}", result);
}
