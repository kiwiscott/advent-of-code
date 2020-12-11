import sys
from copy import deepcopy 

with open(sys.argv[1], 'r') as f:
    contents = f.read()

ALL = [list(l.strip()) for l in list(contents.split("\n"))]

ROWS = len(ALL)

SEATS = len(ALL[0])
first = True



while True: 
    NEW_ALL = deepcopy(ALL)
    changes = False 
    for ri in range(ROWS):
        for si in range(SEATS):
            neigh = 0 
            for rc in [-1,0,1]:
                for sc in [-1,0,1]:
                    if not (rc == 0 and sc ==0):
                        row = ri+rc
                        seat = si+sc
                        
                        while 0<=row<ROWS and 0<=seat<SEATS and ALL[row][seat] == '.':
                            row = row+rc
                            seat = seat+sc
                        
                        if 0<=row<ROWS and 0<=seat<SEATS and ALL[row][seat] == '#':
                            neigh +=1 

            if first  and (ALL[ri][si] == '#' or  ALL[ri][si] == 'L'):
                NEW_ALL[ri][si] = '#'
                changes = True
            elif neigh == 0 and ALL[ri][si] == 'L':
                NEW_ALL[ri][si] = '#'
                changes = True
            elif neigh >= 4 and ALL[ri][si] == '#':
                NEW_ALL[ri][si] = 'L'
                changes = True


    first = False
    ALL = deepcopy(NEW_ALL)

    if not changes:
        break

acc = 0
for ri in range(ROWS):
    for si in range(SEATS):
        if ALL[ri][si] == "#":
            acc+=1

print(acc)   

