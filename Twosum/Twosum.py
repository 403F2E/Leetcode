nums = list(input("enter a list of integer : ").split(" "))
target = int(input("enter ur target : "))
def Solution(nums: list, target: int):
    for i in range(len(nums)):
        for j in range(i+1, len(nums)):
            if nums[i] + nums[j] == target:
                return [i, j]
print(Solution(nums, target))