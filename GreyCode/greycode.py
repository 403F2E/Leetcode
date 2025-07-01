class Solution:
    def grayCode(self, n: int) -> list[int]:
        result: list[int] = []
        
        for i in range(1 << n):  
            gray_code = i ^ (i >> 1) 
            result.append(gray_code)
        
        return result
