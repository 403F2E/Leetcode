
''' 
Given two strings s and t, return true if t is an anagram of s, and false otherwise.
'''

def isAnagram(s: str, t: str) -> bool:

    ''' Complexity O(n) Solution '''
    if len(s) != len(t):
        return False

    letters = set(s)
    for letter in letters:
        s_count, t_count = s.count(letter), t.count(letter)
        if s_count != t_count:
            return False
    return True


    ''' Complexity O(nlog(n)) solution '''
    # if len(s) != len(t):
    #     return False
    #
    # return sorted(s) == sorted(t)
