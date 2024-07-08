struct ArrayBinaryTree {
    tree: Vec<Option<i32>>,
}

impl ArrayBinaryTree {
    fn new(arr: Vec<Option<i32>>) -> Self {
        Self { tree: arr }
    }

    fn size(&self) -> i32 {
        self.tree.len() as i32
    }

    fn val(&self, i : i32) -> Option<i32> {
        if i < 0 || i >= self.size() {
            None
        } else {
            self.tree[i as usize]
        }
    }

    fn left(&self, i: i32) -> i32 {
        2 * i + 1
    } 

    fn right(&self, i: i32) -> i32 {
        2 * i + 2
    }

    fn parent(&self, i: i32) -> i32 {
        (i - 1) / 2
    }

    fn level_order(&self) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..self.size() {
            if let Some(val) = self.val(i) {
                res.push(val)
            }
        }
        res
    }

    fn dfs(&self, i :i32, order: &str, res: &mut Vec<i32>) {
        if self.val(i).is_none() {
            return;
        }
        let val = self.val(i).unwrap();
        if order == "pre" {
            res.push(val);
        }
        self.dfs(self.left(i), order, res);
        if order == "in" {
            res.push(val);
        }
        self.dfs(self.right(i), order, res);
        if order == "post" {
            res.push(val);
        }
    }

    fn pre_order(&self) -> Vec<i32> {
        let mut res = vec![];
        self.dfs(0, "pre", &mut res);
        return res;
    }

    fn in_order(&self) -> Vec<i32> {
        let mut res = vec![];
        self.dfs(0, "in", &mut res);
        res
    }

    fn post_order(&self) -> Vec<i32> {
        let mut res = vec![];
        self.dfs(0, "post", &mut res);
        res
    }
}


fn main() {
    let arr = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        None,
        None,
        Some(12),
        None,
        None,
        Some(15),
    ];

    let abt = ArrayBinaryTree::new(arr);

    let i = 1;
    let l = abt.left(i);
    let r = abt.right(i);
    let p = abt.parent(i);
    println!(
        "\n当前节点的索引为 {} ，值为 {}",
        i,
        if let Some(val) = abt.val(i) {
            format!("{val}")
        } else {
            "null".to_string()
        }
    );
    println!(
        "其左子节点的索引为 {} ，值为 {}",
        l,
        if let Some(val) = abt.val(l) {
            format!("{val}")
        } else {
            "null".to_string()
        }
    );
    println!(
        "其右子节点的索引为 {} ，值为 {}",
        r,
        if let Some(val) = abt.val(r) {
            format!("{val}")
        } else {
            "null".to_string()
        }
    );
    println!(
        "其父节点的索引为 {} ，值为 {}",
        p,
        if let Some(val) = abt.val(p) {
            format!("{val}")
        } else {
            "null".to_string()
        }
    );

    // tralversal tree
    let mut res = abt.level_order();
    println!("\n层序遍历为：{:?}", res);
    res = abt.pre_order();
    println!("前序遍历为：{:?}", res);
    res = abt.in_order();
    println!("中序遍历为：{:?}", res);
    res = abt.post_order();
    println!("后序遍历为：{:?}", res);
}
