# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

from types import Optional


class ListNode:
    def __init__(self, val=0, n=None) -> None:
        self.val = val
        self.next = n


class Solution:
    def mergeTwoLists(self,
        list1: Optional[ListNode],
        list2: Optional[ListNode]) -> Optional[ListNode]:

        if list1 is None:
            return list2
        elif list2 is None:
            return list1

        if list1.val <= list2.val:
            ln = ListNode(list1.val, ListNode())
            list1 = list1.next
        else:
            ln = ListNode(list2.val, ListNode())
            list2 = list2.next

        start = ln

        while list1 is not None and list2 is not None:            
            ln = ln.next
            if list1.val <= list2.val:
                ln.val = list1.val
                list1 = list1.next
            else:
                ln.val = list2.val
                list2 = list2.next
            ln.next = ListNode()

        while list1 is not None:
            ln = ln.next
            ln.val = list1.val
            if list1.next is None:
                ln.next = None
                return start
            list1 = list1.next
            ln.next = ListNode()

        while list2 is not None:
            ln = ln.next
            ln.val = list2.val
            if list2.next is None:
                ln.next = None
                return start
            list2 = list2.next
            ln.next = ListNode()
            
        ln.next = None

        return start

