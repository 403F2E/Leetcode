
'''
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

 1.Open brackets must be closed by the same type of brackets.
 2.Open brackets must be closed in the correct order.
 3.Every close bracket has a corresponding open bracket of the same type.
'''

def isValid(s: str) -> bool:
    op, cl = "({[", ")}]"
    if s[0] in cl:
        return False

    # The stack
    stack: list = []
    for i in range(len(s)):
        if s[i] in op:
            stack.append(op.index(s[i]))
        elif s[i] in cl:
            if not len(stack) or s[i] != cl[stack[-1]]:
                return False
            stack.pop()
    
    # another solution for this part :
    # return len(stack) == 0
    if not len(stack):
        return True
    return False
