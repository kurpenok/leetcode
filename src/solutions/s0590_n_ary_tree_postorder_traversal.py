from __future__ import annotations


class Node:
    def __init__(self, val: int, children: list[Node]):
        self.val = val
        self.children = children


def helper(node: Node | None, elements: list[int]):
    if node is None:
        return

    for children in node.children:
        helper(children, elements)

    elements.append(node.val)


def postorder(root: Node | None) -> list[int]:
    elements = []
    helper(root, elements)
    return elements
