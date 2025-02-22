
'''
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

'''
from collections import defaultdict

def groupAnagram(strs: list[str]) -> list[list[str]]:
    if len(strs) == 1:
        res = []
        res.append(strs)
        return res

    anagroup = defaultdict(list)
    for word in strs:
        sorted_w = ''.join(sorted(word))
        if word in anagroup:
            anagroup[sorted_w].append(word)
        else:
            anagroup[sorted_w] = [word]
    return list(anagroup.values())
