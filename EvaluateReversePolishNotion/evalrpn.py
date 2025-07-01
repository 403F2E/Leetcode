
'''
You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:

 - The valid operators are '+', '-', '*', and '/'.
 - Each operand may be an integer or another expression.
 - The division between two integers always truncates toward zero.
 - There will not be any division by zero.
 - The input represents a valid arithmetic expression in a reverse polish notation.
 - The answer and all the intermediate calculations can be represented in a 32-bit integer.
'''

def evalRPN(tokens: list[str]) -> int:
    stack = []
    tmp = 0

    for token in tokens:
        if token.isdigit() or len(token) > 1:
            stack.append(int(token))
        else:
            if token == '+':
                tmp = stack[-2] + stack[-1]
            if token == '-':
                tmp = stack[-2] - stack[-1]
            if token == '*':
                tmp = stack[-2] * stack[-1]
            if token == '/':
                tmp = int(stack[-2] / stack[-1])
            stack.pop()
            stack.pop()
            stack.append(tmp)

    return stack[0]

