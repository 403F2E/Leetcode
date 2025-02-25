
def isPalindrome(x: int):
    y: str = str(x)
    if x == y[::-1]:
        return True
    else :
        return False

x = int(input("enter an integr : "))
print(isPalindrome(x))
