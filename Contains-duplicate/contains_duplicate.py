'''
  Contains duplicates

  Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

'''

''' Solution with hashmap '''
# from collections import defaultdict
# from typing import List

# def containsDuplicate(nums: List[int]) -> bool:
#     # The defaultdict in pre-imported in leetcode 
#     hm = defaultdict(int)
#
#     for num in nums:
#         hm[num] += 1
#         if hm[num]:
#             return True
#     return False

''' Solution with set '''
def containsDuplicate(nums: list[int]) -> bool:

    return len(nums) != len(set(nums))
