from itertools import product

'''
Эта штука вроде как быстрее и эффективнее.
Можно попробовать переписать её на расте, ссылки я приложил

'''


'''
References
https://habr.com/ru/post/462411/

https://www.cs.mcgill.ca/~aassaf9/python/algorithm_x.html

'''

def solve_sudoku(size, grid):
    X = ([("row,column", row,column) for row,column in product(range(9), range(9))] + # в каждой строке стоит цифра
         [("row,number", row,number) for row,number in product(range(9), range(1, 9 + 1))] + # в каждой строке по 9 чисел
         [("column,number", column,number) for column,number in product(range(9), range(1, 9 + 1))] + # в каждом столбце по 9 чисел
         [("box,number", box,number) for box,number in product(range(9), range(1, 9 + 1))]) # в каждом боксе по 9 чисел

    Y = dict()
    for row, column, number in product(range(9), range(9), range(1, 9 + 1)): # всевозможные значения судоку
        box = (row // 3) * 3 + (column // 3)  # Box number
        Y.update({
            (row, column, number): [
                ("row,column", (row, column)),
                ("row,number", (row, number)),
                ("column,number", (column, number)),
                ("box,number", (box, number))
            ]
        })
    X, Y = exact_cover(X, Y)

    for i, row in enumerate(grid):
        for j, number in enumerate(row):
            if number:
                select(X, Y, (i, j, number)) # массив столбцов взятых в решение
                # удаляем те позиции которые мы уже знаем
    for solution in solve(X, Y, []):
        for (row, column, number) in solution:
            grid[row][column] = number
        yield grid


def exact_cover(X, Y):
    X = {j: set() for j in X}
    for i, row in Y.items():
        for j in row:
            X[j].add(i)
    return X, Y


def solve(X, Y, solution):
    if not X: # пустой X == решение найдено
        yield solution.clone()
    else:
        c = min(X, key=lambda c: len(X[c])) # выбираем столбец с минимальным кол-вом строк
        for r in list(X[c]):
            solution.append(r) # берем строку в стек решения
            cols = select(X, Y, r) # получаем столбци которые не валидны при строке r
            for s in solve(X, Y, solution): # пытаемся решить задачу без строки r и связаных с ней столбцов
                yield s
            deselect(X, Y, r, cols)
            solution.pop()


def select(X, Y, r): # r = A
    cols = []
    for j in Y[r]: # j = 24
        for i in X[j]: # i = D
            for k in Y[i]:
                if k != j:
                    X[k].remove(i)
        cols.append(X.pop(j))
    return cols


def deselect(X, Y, r, cols):
    for j in reversed(Y[r]):
        X[j] = cols.pop() # возвращаем то что мы удалили при селекте в строке 77
        for i in X[j]:
            for k in Y[i]:
                if k != j:
                    X[k].add(i) # возвращаем содержимое строки


def string_to_grid(s):
    grid = [[ 0 for i in range(9)] for j in range(9)]
    for i in range(len(s)):
        grid[i // 9][i % 9] = int(s[i])
    return grid

def grid_to_string(g):
    return ''.join([''.join(list(map(str, r))) for r in g])


def apply_mask(s ,m):
    s = list(s)
    m = list(m)
    for i in range(len(s)):
        s[i] = s[i] if m[i] == '1' else '0'
    return ''.join(s)

# ''' Ver 1
# Ex In:
# 568010020040700010007400680026043870000620049473100200700000008685070002200005704
# '''
grid =  string_to_grid(input())
for i in grid:
    print(i)

# ''' Ver 2
# Ex In:
# 568319427342786915197452683926543871851627349473198256734261598685974132219835764
# 111010010010100010001100110011011110000110011111100100100000001111010001100001101
# '''
# solution = input()
# mask = input()
# grid = string_to_grid(apply_mask(solution, mask))
try:
    for solution in solve_sudoku((3, 3), grid):
        print(grid_to_string(solution))
except:
    print('Impossible to solve')