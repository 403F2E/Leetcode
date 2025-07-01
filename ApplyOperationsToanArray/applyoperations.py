
'''
You are given a 0-indexed array nums of size n consisting of non-negative integers.

You need to apply n - 1 operations to this array where, in the ith operation (0-indexed), you will apply the following on the ith element of nums:

 - If nums[i] == nums[i + 1], then multiply nums[i] by 2 and set nums[i + 1] to 0. Otherwise, you skip this operation.
    After performing all the operations, shift all the 0's to the end of the array.

 - For example, the array [1,0,2,0,0,1] after shifting all its 0's to the end, is [1,2,1,0,0,0].
 Return the resulting array.

    Note that the operations are applied sequentially, not all at once.
'''

def applyoperations(nums: list[int]) -> list[int]:
    n: int = len(nums)
    s, f = 0, 1

    # the loop that execute the first application of (* 2 or skip)
    for i in range(n - 1):
        if nums[i] == nums[i + 1] != 0:
            nums[i] *= 2
            nums[i + 1] = 0

    # the loop that execute the second application of shifting the zeros to the right
    for f in range(n):
        if nums[s] != 0:
            s += 1
        else:
            if nums[f] != 0:
                nums[s] = nums[f]
                nums[f] = 0
                s += 1

        f += 1

    # another way to implement the loop before is :
    '''
    for f in range(n):
        if nums[f] != 0:
            nums[f], nums[s] = nums[f], nums[s]
            s += 1
    '''



    return nums
