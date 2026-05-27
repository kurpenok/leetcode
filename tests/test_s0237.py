from src.list import ListNode
from src.s0237_delete_node_in_a_linked_list import delete_node


def test_case_1():
    head = ListNode(4)
    head.next = ListNode(5)
    head.next.next = ListNode(1)
    head.next.next.next = ListNode(9)

    delete_node(head.next)

    assert head.val == 4
    assert head.next.val == 1
    assert head.next.next.val == 9


def test_case_2():
    head = ListNode(4)
    head.next = ListNode(5)
    head.next.next = ListNode(1)
    head.next.next.next = ListNode(9)

    delete_node(head.next.next)

    assert head.val == 4
    assert head.next.val == 5
    assert head.next.next.val == 9
