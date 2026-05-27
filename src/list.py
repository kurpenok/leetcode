from __future__ import annotations


class ListNode:
    def __init__(self, x: int):
        self.val: int = x
        self.next: ListNode | None = None
