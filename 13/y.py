with open('input.txt') as file:
    earliest = int(file.readline())
    buses = [(index, int(bus)) for index, bus in enumerate(file.readline().split(',')) if bus != 'x']

time, step, count  = 0, 1,0
print(buses)

buses.sort(key=lambda tup: tup[1], reverse=True)
print(buses)

for offset, bus in buses:
    while (time + offset) % bus:
        count+=1
        time += step
    
    step *= bus

print(time,count)