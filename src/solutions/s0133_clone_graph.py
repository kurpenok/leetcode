from __future__ import annotations

from collections import deque


class Node:
    def __init__(self, val: int = 0, neighbors: list[Node] | None = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Node | None) -> Node | None:
        if not node:
            return node

        q = deque([node])
        clones = {node.val: Node(node.val, [])}
        while q:
            current = q.popleft()
            current_clone = clones[current.val]

            for neighbor in current.neighbors:
                if neighbor.val not in clones:
                    clones[neighbor.val] = Node(neighbor.val, [])
                    q.append(neighbor)

                current_clone.neighbors.append(clones[neighbor.val])

        return clones[node.val]
