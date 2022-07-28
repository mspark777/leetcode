"""
solution
"""
from __future__ import annotations
from collections import Counter
from typing import  Optional

class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return Counter(s) == Counter(t)
