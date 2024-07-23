use std::cell::Cell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

struct BianrySearchTree {
    root: OptionTreeNodeRc,
}

impl BianrySearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn get_root(&self) -> OptionTreeNodeRc {
        self.root.clone()
    }

    fn search(&self, val: i32) -> OptionTreeNodeRc {
        if self.root.is_none() {
            return None;
        }

        let mut cur = self.root.clone();
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Equal => break,
                Ordering::Greater => cur = node.borrow().right.clone(),
                Ordering::Less => cur = node.borrow().left.clone(),
            }
        }
        cur
    }

    fn insert(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Some(TreeNode::new(val));
        }

        let mut cur = self.root.clone();
        let mut pre = None;
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Equal => return,
                Ordering::Greater => {
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                }
                Ordering::Less => {
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                }
            }
        }
        let pre = pre.unwrap();
        let node = Some(TreeNode::new(val));
        if pre.borrow().val < val {
            pre.borrow_mut().right = node;
        } else {
            pre.borrow_mut().left = node;
        }
    }

    fn remove(&mut self, val: i32) {
        if self.root.is_none() {
            return;
        }

        let mut cur = self.root.clone();
        let mut pre = None;
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Equal => break,
                Ordering::Greater => {
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                }
                Ordering::Less => {
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                }
            }
        }

        if cur.is_none() {
            return;
        }

        let cur = cur.unwrap();
        let (left_child, right_child) = (cur.borrow().left.clone(), cur.borrow().right.clone());
        match (left_child.clone(), right_child.clone()) {
            (None, None) | (Some(_), None) | (None, Some(_)) => {
                let child = left_child.or(right_child);
                let pre = pre.unwrap();
                if !Rc::ptr_eq(&cur, self.root.as_ref().unwrap()) {
                    let left = pre.borrow().left.clone();
                    if left.is_some() && Rc::ptr_eq(&left.as_ref().unwrap(), &cur) {
                        pre.borrow_mut().left = child;
                    } else {
                        pre.borrow_mut().right = child;
                    }
                } else {
                    self.root = child;
                }
            }
            (Some(_), Some(_)) => {
                let mut tmp = cur.borrow().right.clone();
                while let Some(node) = tmp.clone() {
                    if node.borrow().left.is_some() {
                        tmp = node.borrow().left.clone();
                    } else {
                        break;
                    }
                }
                let tmpval = tmp.unwrap().borrow().val;
                self.remove(tmpval);
                cur.borrow_mut().val = tmpval;
            }
        }
    }
}

struct Trunk<'a, 'b> {
    prev: Option<&'a Trunk<'a, 'b>>,
    str: Cell<&'b str>,
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

fn main() {
    let mut bst = BianrySearchTree::new();

    let nums = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
    for &num in &nums {
        bst.insert(num);
    }

    println!("\n初始化的二叉树为\n");
    print_tree(bst.get_root().as_ref().unwrap());

    /* 查找结点 */
    let node = bst.search(7);
    println!(
        "\n查找到的节点对象为 {:?}，节点值 = {}",
        node.clone().unwrap(),
        node.clone().unwrap().borrow().val
    );

    /* 插入节点 */
    bst.insert(16);
    println!("\n插入节点 16 后，二叉树为\n");
    print_tree(bst.get_root().as_ref().unwrap());

    /* 删除节点 */
    bst.remove(1);
    println!("\n删除节点 1 后，二叉树为\n");
    print_tree(bst.get_root().as_ref().unwrap());
    bst.remove(2);
    println!("\n删除节点 2 后，二叉树为\n");
    print_tree(bst.get_root().as_ref().unwrap());
    bst.remove(4);
    println!("\n删除节点 4 后，二叉树为\n");
    print_tree(bst.get_root().as_ref().unwrap());
}
