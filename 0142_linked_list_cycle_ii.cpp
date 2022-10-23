struct ListNode {
    int val;
    ListNode* next;
};

class Solution {
public:
    ListNode* detectCycle(ListNode* head) {
        ListNode* start = head;
        int indicator = 1000000;
        int pos = 0;

        while (true) {
            if (head == nullptr) {
                break;
            } else if (head->val < -indicator) {
                head->val += indicator;
                break;
            } else if (head->val > indicator) {
                head->val -= indicator;
                break;
            } else if (head->val < 0) {
                head->val -= indicator;
            } else if (head->val >= 0) {
                head->val += indicator;
            }

            head = head->next;
            pos++;
        }

        while (true) {
            if (start == nullptr) {
                break;
            } else if (start->val < -indicator) {
                start->val += indicator;
            } else if (start->val > indicator) {
                start->val -= indicator;
            } else {
                break;
            }

            start = start->next;
        }

        return start;
    }
};
