struct ListNode {
    int val;
    ListNode* next;
    explicit ListNode(int x): val(x), next(nullptr) {}
};

class Solution {
public:
    bool hasCycle(ListNode* head) {
        ListNode* start = head;
        int indicator = 1000000;

        while (true) {
            if (head == nullptr) {
                return false;
            } else if (head->val < -indicator) {
                return true;
            } else if (head->val > indicator) {
                return true;
            } else if (head->val < 0) {
                head->val -= indicator;
            } else if (head->val >= 0) {
                head->val += indicator;
            }

            head = head->next;
        }

        return false;
    }
};
