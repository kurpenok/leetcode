from __future__ import annotations


class Node:
    def __init__(self, val: int, children: list[Node]):
        self.val = val
        self.children = children


def helper(node: Node | None, elements: list[int]):
    if node is None:
        return

    elements.append(node.val)
    for children in node.children:
        helper(children, elements)


def preorder(root: Node) -> list[int]:
    elements = []
    helper(root, elements)
    return elements
