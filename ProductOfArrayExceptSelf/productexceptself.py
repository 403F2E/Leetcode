
'''
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation
'''


def productExceptSelf(nums: list[int]) -> list[int]:
    n = len(nums)
    prefix, suffix = [0] * n, [0] * n
    answer: list[int] = [0] * n
    prefix[0], suffix[0] = nums[0], nums[n - 1]


    for i in range(1, n):
        prefix[i] = prefix[i - 1] * nums[i]
        suffix[i] = suffix[i - 1] * nums[n - i - 1]

    for i in range(1, n-1):
        answer[i] = prefix[i - 1] * suffix[n - i - 2]

    answer[-1] = prefix[n - 2]
    answer[0] = suffix[n - 2]

    return answer

