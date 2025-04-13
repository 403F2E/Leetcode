
'''
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.
'''
def isPalindrome(self, s: str) -> bool:
    import re
    s = s.lower()
    s = re.sub("[\s_\W]", "", s)
    return s == s[::-1]
