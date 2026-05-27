from src.list import ListNode


def delete_node(node: ListNode) -> None:
    node.val = node.next.val if node.next else 0
    node.next = node.next.next if node.next else None
