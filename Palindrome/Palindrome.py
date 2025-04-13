
'''
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.
'''
def isPalindrome(s: str) -> bool:
    import re
    s = re.sub(r'[^a-zA-Z0-9]', "", s).lower()
    return s == s[::-1]
