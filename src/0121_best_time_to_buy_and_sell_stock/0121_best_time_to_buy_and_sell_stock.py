#!/usr/bin/env python3

from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        local_min = prices[0]

        for i in range(len(prices) - 1):
            local_min = min(local_min, prices[i])
            profit = max(prices[i + 1] - local_min, profit)

        return profit
