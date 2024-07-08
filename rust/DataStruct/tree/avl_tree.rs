use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    height: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(Self {
            val,
            height: 0,
            left: None,
            right: None,
        }))
    }
}

type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

struct AVLTree {
    root: OptionTreeNodeRc,
}

impl AVLTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn height(node: OptionTreeNodeRc) -> i32 {
        match node {
            Some(node) => node.borrow().height,
            None => -1,
        }
    }

    fn update_height(node: OptionTreeNodeRc) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1;
        }
    }

    fn balance_factor(node: OptionTreeNodeRc) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
        }
    }

    fn right_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_child = child.borrow().right.clone();
                child.borrow_mut().right = Some(node.clone());
                node.borrow_mut().left = grand_child;
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn left_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_child = child.borrow().left.clone();
                child.borrow_mut().left = Some(node.clone());
                node.borrow_mut().right = grand_child;
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        let balance_factor = Self::balance_factor(node.clone());
        if balance_factor > 1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        } else if balance_factor < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            node
        }
    }

    fn insert(&mut self, val: i32) {
        self.root = Self::insert_helper(self.root.clone(), val);
    }

    fn insert_helper(node: OptionTreeNodeRc, val: i32) -> OptionTreeNodeRc {
        match node {
            Some(mut node) => {
                match {
                    let node_val = node.borrow().val;
                    node_val
                }
                .cmp(&val)
                {
                    Ordering::Greater => {
                        let left = node.borrow().left.clone();
                        node.borrow_mut().left = Self::insert_helper(left, val);
                    }
                    Ordering::Less => {
                        let right = node.borrow().right.clone();
                        node.borrow_mut().right = Self::insert_helper(right, val);
                    }
                    Ordering::Equal => {
                        return Some(node);
                    }
                }
                Self::update_height(Some(node.clone()));
                node = Self::rotate(Some(node)).unwrap();
                Some(node)
            }
            None => Some(TreeNode::new(val)),
        }
    }

    fn remove(&self, val: i32) {
        Self::remove_helper(self.root.clone(), val);
    }

    fn remove_helper(node: OptionTreeNodeRc, val: i32) -> OptionTreeNodeRc {
        match node {
            Some(mut node) => {
                if val < node.borrow().val {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::remove_helper(left, val);
                } else if val > node.borrow().val {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_helper(right, val);
                } else if node.borrow().left.is_none() || node.borrow().right.is_none() {
                    let child = if node.borrow().left.is_some() {
                        node.borrow().left.clone()
                    } else {
                        node.borrow().right.clone()
                    };
                    match child {
                        None => {
                            return None;
                        }
                        Some(child) => node = child,
                    }
                } else {
                    let mut temp = node.borrow().right.clone().unwrap();
                    loop {
                        let temp_left = temp.borrow().left.clone();
                        if temp_left.is_none() {
                            break;
                        }
                        temp = temp_left.unwrap();
                    }
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_helper(right, temp.borrow().val);
                    node.borrow_mut().val = temp.borrow().val;
                }
                Self::update_height(Some(node.clone()));

                node = Self::rotate(Some(node)).unwrap();
                Some(node)
            }
            None => None,
        }
    }

    fn search(&self, val: i32) -> OptionTreeNodeRc {
        let mut cur = self.root.clone();
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Greater => {
                    cur = node.borrow().right.clone();
                }
                Ordering::Less => {
                    cur = node.borrow().left.clone();
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
        cur
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




fn main() {
    fn test_insert(tree: &mut AVLTree, val: i32) {
        tree.insert(val);
        println!("\n插入节点 {} 后，AVL 树为", val);
        print_tree(&tree.root.clone().unwrap());
    }

    fn test_remove(tree: &mut AVLTree, val: i32) {
        tree.remove(val);
        println!("\n删除节点 {} 后，AVL 树为", val);
        print_tree(&tree.root.clone().unwrap());
    }

    /* 初始化空 AVL 树 */
    let mut avl_tree = AVLTree::new();

    /* 插入节点 */
    // 请关注插入节点后，AVL 树是如何保持平衡的
    test_insert(&mut avl_tree, 1);
    test_insert(&mut avl_tree, 2);
    test_insert(&mut avl_tree, 3);
    test_insert(&mut avl_tree, 4);
    test_insert(&mut avl_tree, 5);
    test_insert(&mut avl_tree, 8);
    test_insert(&mut avl_tree, 7);
    test_insert(&mut avl_tree, 9);
    test_insert(&mut avl_tree, 10);
    test_insert(&mut avl_tree, 6);

    /* 插入重复节点 */
    test_insert(&mut avl_tree, 7);

    /* 删除节点 */
    // 请关注删除节点后，AVL 树是如何保持平衡的
    test_remove(&mut avl_tree, 8); // 删除度为 0 的节点
    test_remove(&mut avl_tree, 5); // 删除度为 1 的节点
    test_remove(&mut avl_tree, 4); // 删除度为 2 的节点

    /* 查询节点 */
    let node = avl_tree.search(7);
    if let Some(node) = node {
        println!(
            "\n查找到的节点对象为 {:?}，节点值 = {}",
            &*node.borrow(),
            node.borrow().val
        );
    }
}
