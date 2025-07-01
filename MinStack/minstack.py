
'''
Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

 - MinStack() initializes the stack object.
 - void push(int val) pushes the element val onto the stack.
 - void pop() removes the element on the top of the stack.
 - int top() gets the top element of the stack.
 - int getMin() retrieves the minimum element in the stack.
You must implement a solution with O(1) time complexity for each function.
'''

class MinStack:

    def __init__(self):
        self.stack = []
        self.min_ = float('inf')

    def push(self, val: int) -> None:
        self.stack.append(val)
        if self.min_ > val:
            self.min_ = val

    def pop(self) -> None:
        if self.min_ == self.stack[len(self.stack)-1]:
            self.stack.pop()
            if self.stack:
                self.min_ = min(self.stack)
            else:
                self.min_ = float('inf')
        else:
            self.stack.pop()

    def top(self) -> int:
        return self.stack[len(self.stack)-1]

    def getMin(self) -> int:
        return int(self.min_)
