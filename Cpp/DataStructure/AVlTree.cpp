#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct TreeNode {
  int val{};
  int height = 0;
  TreeNode *parent{};
  TreeNode *left{};
  TreeNode *right{};
  TreeNode() = default;
  explicit TreeNode(int x, TreeNode *parent = nullptr)
      : val(x), parent(parent) {}
};

void freeMemoryTree(TreeNode *root) {
  if (root == nullptr) {
    return;
  }
  freeMemoryTree(root->left);
  freeMemoryTree(root->right);
  delete root;
}

class AVLTree {
private:
  void updateHeight(TreeNode *node) {
    node->height = std::max(height(node->left), height(node->right)) + 1;
  }
  /* 右旋操作 */
  TreeNode *rightRotate(TreeNode *node) {
    TreeNode *child = node->left;
    TreeNode *grandChild = child->right;
    // 以 child 为原点，将 node 向右旋转
    child->right = node;
    node->left = grandChild;
    // 更新节点高度
    updateHeight(node);
    updateHeight(child);
    // 返回旋转后子树的根节点
    return child;
  }

  /* 左旋操作 */
  TreeNode *leftRotate(TreeNode *node) {
    TreeNode *child = node->right;
    TreeNode *grandChild = child->left;
    // 以 child 为原点，将 node 向左旋转
    child->left = node;
    node->right = grandChild;
    // 更新节点高度
    updateHeight(node);
    updateHeight(child);
    // 返回旋转后子树的根节点
    return child;
  }

  /* 执行旋转操作，使该子树重新恢复平衡 */
  TreeNode *rotate(TreeNode *node) {
    // 获取节点 node 的平衡因子
    int _balanceFactor = balanceFactor(node);
    // 左偏树
    if (_balanceFactor > 1) {
      if (balanceFactor(node->left) >= 0) {
        // 右旋
        return rightRotate(node);
      } else {
        // 先左旋后右旋
        node->left = leftRotate(node->left);
        return rightRotate(node);
      }
    }
    // 右偏树
    if (_balanceFactor < -1) {
      if (balanceFactor(node->right) <= 0) {
        // 左旋
        return leftRotate(node);
      } else {
        // 先右旋后左旋
        node->right = rightRotate(node->right);
        return leftRotate(node);
      }
    }
    // 平衡树，无须旋转，直接返回
    return node;
  }

  TreeNode *insertHelper(TreeNode *node, int val) {
    if (node == nullptr)
      return new TreeNode(val);
    /* 1. 查找插入位置并插入节点 */
    if (val < node->val)
      node->left = insertHelper(node->left, val);
    else if (val > node->val)
      node->right = insertHelper(node->right, val);
    else
      return node;      // 重复节点不插入，直接返回
    updateHeight(node); // 更新节点高度
    /* 2. 执行旋转操作，使该子树重新恢复平衡 */
    node = rotate(node);
    // 返回子树的根节点
    return node;
  }

  TreeNode *removeHelper(TreeNode *node, int val) {
    if (node == nullptr) {
      return nullptr;
    }
    if (val < node->val) {
      node->left = removeHelper(node->left, val);
    } else if (val > node->val) {
      node->right = removeHelper(node->right, val);
    } else {
      if (node->left == nullptr || node->right == nullptr) {
        TreeNode *child = node->left != nullptr ? node->left : node->right;
        if (child == nullptr) {
          delete node;
          return nullptr;
        } else {
          delete node;
          node = child;
        }
      } else {
        TreeNode *temp = node->right;
        while (temp->left != nullptr) {
          temp = temp->left;
        }
        int tempVal = temp->val;
        node->right = removeHelper(node->right, tempVal);
        node->val = tempVal;
      }
    }
    updateHeight(node);
    node = rotate(node);
    return node;
  }

public:
  TreeNode *root;

  int height(TreeNode *node) { return node == nullptr ? -1 : node->height; }

  int balanceFactor(TreeNode *node) {
    if (node == nullptr) {
      return 0;
    }
    return height(node->left) - height(node->right);
  }

  void insert(int val) { root = insertHelper(root, val); }

  void remove(int val) { root = removeHelper(root, val); }

  TreeNode *search(int val) {
    TreeNode *cur = root;
    while (cur != nullptr) {
      if (cur->val < val) {
        cur = cur->right;
      } else if (cur->val > val) {
        cur = cur->left;
      } else {
        break;
      }
    }
    return cur;
  }

  AVLTree() : root(nullptr) {}

  ~AVLTree() { freeMemoryTree(root); }
};

struct Trunk {
  Trunk *prev;
  string str;
  Trunk(Trunk *prev, string str) {
    this->prev = prev;
    this->str = str;
  }
};

void showTrunks(Trunk *p) {
  if (p == nullptr) {
    return;
  }

  showTrunks(p->prev);
  cout << p->str;
}

/**
 * 打印二叉树
 * This tree printer is borrowed from TECHIE DELIGHT
 * https://www.techiedelight.com/c-program-print-binary-tree/
 */
void printTree(TreeNode *root, Trunk *prev, bool isRight) {
  if (root == nullptr) {
    return;
  }

  string prev_str = "    ";
  Trunk trunk(prev, prev_str);

  printTree(root->right, &trunk, true);

  if (!prev) {
    trunk.str = "———";
  } else if (isRight) {
    trunk.str = "/———";
    prev_str = "   |";
  } else {
    trunk.str = "\\———";
    prev->str = prev_str;
  }

  showTrunks(&trunk);
  cout << " " << root->val << endl;

  if (prev) {
    prev->str = prev_str;
  }
  trunk.str = "   |";

  printTree(root->left, &trunk, false);
}

/* 打印二叉树 */
void printTree(TreeNode *root) { printTree(root, nullptr, false); }

void testInsert(AVLTree &tree, int val) {
  tree.insert(val);
  std::cout << "\n插入节点 " << val << " 后，AVL 树为" << std::endl;
  printTree(tree.root);
}

void testRemove(AVLTree &tree, int val) {
  tree.remove(val);
  std::cout << "\n删除节点 " << val << " 后，AVL 树为" << std::endl;
  printTree(tree.root);
}

int main() {
  /* 初始化空 AVL 树 */
  AVLTree avlTree;

  /* 插入节点 */
  // 请关注插入节点后，AVL 树是如何保持平衡的
  testInsert(avlTree, 1);
  testInsert(avlTree, 2);
  testInsert(avlTree, 3);
  testInsert(avlTree, 4);
  testInsert(avlTree, 5);
  testInsert(avlTree, 8);
  testInsert(avlTree, 7);
  testInsert(avlTree, 9);
  testInsert(avlTree, 10);
  testInsert(avlTree, 6);

  /* 插入重复节点 */
  testInsert(avlTree, 7);

  /* 删除节点 */
  // 请关注删除节点后，AVL 树是如何保持平衡的
  testRemove(avlTree, 8); // 删除度为 0 的节点
  testRemove(avlTree, 5); // 删除度为 1 的节点
  testRemove(avlTree, 4); // 删除度为 2 的节点

  /* 查询节点 */
  TreeNode *node = avlTree.search(7);
  std::cout << "\n查找到的节点对象为 " << node << "，节点值 = " << node->val
            << std::endl;
}
