# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        n1 = ""
        n2 = ""

        e1 = l1
        e2 = l2

        while True:
            if e1.next is None:
                n1 += str(e1.val)
                break
            n1 += str(e1.val)
            e1 = e1.next

        while True:
            if e2.next is None:
                n2 += str(e2.val)
                break
            n2 += str(e2.val)
            e2 = e2.next
            
        n1 = n1[::-1]
        n2 = n2[::-1]

        n = list(reversed(list(str(int(n1) + int(n2)))))
        
        if len(n) == 1:
            return ListNode(n[0], None)

        ln = ListNode(n[0], ListNode())
        start = ln

        for i in range(len(n)):
            if not i:
                continue
                
            if i == len(n) - 1:
                ln = ln.next
                ln.val = n[i]
                ln.next = None
                return start

            ln = ln.next
            ln.val = n[i]
            ln.next = ListNode()

