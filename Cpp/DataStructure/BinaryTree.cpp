#include <iostream>
#include <string>

struct TreeNode {
    int val{};
    int height = 0;
    TreeNode* parent{};
    TreeNode* left{};
    TreeNode* right{};
    TreeNode() = default;
    explicit TreeNode(int val_, TreeNode* parent_=nullptr) : val(val_), parent(parent_) {} 
};

struct Trunk {
    Trunk *prev;
    std::string str;

    Trunk(Trunk* prev, std::string str)
    {
        this->prev = prev;
        this->str = str;
    }
};

void showTrunk(Trunk* p) {
    if (p == nullptr) {
        return;
    }

    showTrunk(p->prev);
    std::cout << p->str;
}

void printTree(TreeNode* node, Trunk* prev, bool isLeft) {
    if (node == nullptr) {
        return;
    }

    std::string prev_str = "    ";
    Trunk* trunk = new Trunk(prev, prev_str);

    printTree(node->right, trunk, true);

    if (!prev) {
        trunk->str = "---";
    } 
    else if (isLeft) {
        trunk->str = ".---";
        prev_str = "   |";
    }
    else {
        trunk->str = "`---";
        prev->str = prev_str;
    }

    showTrunk(trunk);
    std::cout << " " << node->val << std::endl;

    if (prev) {
        prev->str = prev_str;
    }
    trunk->str = "   |";

    printTree(node->left, trunk, false);
}

// PostOrder
void freeMemoryTree(TreeNode* node) {
    if (node == nullptr) {
        return;
    }
    freeMemoryTree(node->left);
    freeMemoryTree(node->right);

    delete node;
}

void preOrder(TreeNode* node) {
    if (node == nullptr) {
        std::cout << "The node is nullllllll\n";
        return;
    }
    std::cout << node->val << std::endl;
    preOrder(node->left);
    preOrder(node->right);
}

void printTree(TreeNode* node) {
    printTree(node, nullptr, false);
}

void level(TreeNode* node) {
    
}

int main() {
    TreeNode* n1 = new TreeNode(1); // memory alloc and call constructor
    TreeNode* n2 = new TreeNode(2);
    TreeNode* n3 = new TreeNode(3);
    TreeNode* n4 = new TreeNode(4);
    TreeNode* n5 = new TreeNode(5);

    n1->left = n2;
    n1->right = n3;
    n2->left = n4;
    n2->right = n5;


    preOrder(n1);
    printTree(n1);
}