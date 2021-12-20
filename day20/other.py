sensorData = open('./input.txt').read().splitlines()

diff = [-1,0,1]

key = None
trenchMap = set()
y = 0
for line in sensorData:
    if not key:
        key = line
    elif len(line) > 0:
        for x in range(len(line)):
            if line[x] == '#':
                trenchMap.add((x,y))
        y += 1

neighboorhood = set()
for point in trenchMap:
    x,y = point
    for dx in diff:
        for dy in diff:
            neighboorhood.add((x+dx,y+dy))

print(len(key))
print(len(trenchMap))


def oneStep(trenchMap, neighboorhood, itr=0):
    itrKey = {'#':'1', '.':'0'}
    pointToSave = '#'
    if key[0] == '#' and key[-1] == '.':
        if itr > 0 and (itr%2) == 0:
            itrKey = {'#':'0', '.':'1'}
        else:
            pointToSave = '.'
    newTrenchMap = set()
    newNeighboorhood = set()
    for point in neighboorhood:
        number = ''
        x,y = point
        for dy in diff:
            for dx in diff:
                if (x+dx, y+dy) in trenchMap:
                    number += itrKey['#']
                else:
                    number += itrKey['.']
        if key[int(number,2)] == pointToSave:
            newTrenchMap.add(point)
            for dy in diff:
                for dx in diff:
                    newNeighboorhood.add((x+dx,y+dy))
                    
    

    return newTrenchMap, newNeighboorhood

def printMap(trenchMap):
    minY = len(trenchMap)
    minX = len(trenchMap)
    maxY = 0
    maxX = 0
    for point in trenchMap:
        x,y = point
        if y < minY:
            minY = y
        if y > maxY:
            maxY = y
        if x < minX:
            minX = x
        if x > maxX:
            maxX = x
    for y in range(minY,maxY+1):
        for x in range(minX,maxX+1):
            if (x,y) in trenchMap:
                print('#',end='')
            else:
                print('.',end='')
        print('')
    print('')

for i in range(1):
    trenchMap, neighboorhood = oneStep(trenchMap, neighboorhood, itr = i+1)
    printMap(trenchMap)
    if (i+1) == 2:
        print("Answer part A: the number of lit points is {}".format(len(trenchMap)))
print("Answer part B: the number of lit points is {}".format(len(trenchMap)))

