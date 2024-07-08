#ifndef BPLUSTREE_H
#define BPLUSTREE_H

class BPlusNode {
public:
    BPlusNode(int order_): isLeaf(false) {
        const szie_t m = order_ * 2;
        key = new int[m];
        
    };
    ~BPlusNode() = default;
private:
    int* key;
    BPlusNode* children;
    bool isLeaf;
};

class BPlusTree {
public:
    BPlusTree(int order_): order(order_) {
        if (root == NULL) {
            root = &BPlusNode(order);
        }
    } 
    ~BPlusTree() = default;

    void insert(int key_, int value_) {}
    BPlusNode* search() {}
private:

private:
    int order;
    BPlusNode *root;
};

#endif