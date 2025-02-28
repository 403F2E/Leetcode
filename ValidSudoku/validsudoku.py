
'''
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

 - Each row must contain the digits 1-9 without repetition.
 - Each column must contain the digits 1-9 without repetition.
 - Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
    
    Note:
            A Sudoku board (partially filled) could be valid but is not necessarily solvable.
        only the filled cells need to be validated according to the mentioned rules.
'''

def isValidSudoku(board: list[list[str]]) -> bool:
    rows: list[str] = []
    cols: list[list[str]] = [[] for _ in range(9)]
    boxes: list[list[str]] = [[] for _ in range(9)]

    for i in range(9):
        rows = [' ']
        for j in range(9):
            if board[i][j] != ".":
                if board[i][j] in rows or board[i][j] in cols[j] or board[i][j] in boxes[(i // 3) * 3 + (j // 3)]:
                    return False

                rows.append(board[i][j])
                cols[j].append(board[i][j])
                boxes[(i // 3) * 3 + (j // 3)].append(board[i][j])

    return True

