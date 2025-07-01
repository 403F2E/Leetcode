
'''
Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
'''


from collections import Counter

def topKFrequent(nums: list[int], k: int) -> list[int]:
    
    # the Counter  class is pre-imported in leetcode otherwise import it
    counter = Counter(nums)
    counter = counter.most_common(k)
    topKF = [element[0] for element in counter]
    return topKF
