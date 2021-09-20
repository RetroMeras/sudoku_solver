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
    R, C = size
    N = R * C
    X = ([("rc", rc) for rc in product(range(N), range(N))] +
         [("rn", rn) for rn in product(range(N), range(1, N + 1))] +
         [("cn", cn) for cn in product(range(N), range(1, N + 1))] +
         [("bn", bn) for bn in product(range(N), range(1, N + 1))])
    Y = dict()
    for r, c, n in product(range(N), range(N), range(1, N + 1)):
        b = (r // R) * R + (c // C)  # Box number
        Y[(r, c, n)] = [
            ("rc", (r, c)),
            ("rn", (r, n)),
            ("cn", (c, n)),
            ("bn", (b, n))]
    X, Y = exact_cover(X, Y)
    for i, row in enumerate(grid):
        for j, n in enumerate(row):
            if n:
                select(X, Y, (i, j, n))
    for solution in solve(X, Y, []):
        for (r, c, n) in solution:
            grid[r][c] = n
        yield grid


def exact_cover(X, Y):
    X = {j: set() for j in X}
    for i, row in Y.items():
        for j in row:
            X[j].add(i)
    return X, Y


def solve(X, Y, solution):
    if not X:
        yield list(solution)
    else:
        c = min(X, key=lambda c: len(X[c]))
        for r in list(X[c]):
            solution.append(r)
            cols = select(X, Y, r)
            for s in solve(X, Y, solution):
                yield s
            deselect(X, Y, r, cols)
            solution.pop()


def select(X, Y, r):
    cols = []
    for j in Y[r]:
        for i in X[j]:
            for k in Y[i]:
                if k != j:
                    X[k].remove(i)
        cols.append(X.pop(j))
    return cols


def deselect(X, Y, r, cols):
    for j in reversed(Y[r]):
        X[j] = cols.pop()
        for i in X[j]:
            for k in Y[i]:
                if k != j:
                    X[k].add(i)


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



''' Ver 1
Ex In:
568010020040700010007400680026043870000620049473100200700000008685070002200005704
'''
grid =  string_to_grid(input())

''' Ver 2
Ex In:
568319427342786915197452683926543871851627349473198256734261598685974132219835764
111010010010100010001100110011011110000110011111100100100000001111010001100001101
'''
# solution = input()
# mask = input()
# grid = string_to_grid(apply_mask(solution, mask))
try:
    for solution in solve_sudoku((3, 3), grid):
        print(grid_to_string(solution))
except:
    print('Impossible to solve')
