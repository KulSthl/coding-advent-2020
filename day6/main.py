def handle_part_1(cache, counter):
    dict = []
    for line in cache:
        for char in line:
            if char not in dict:
                dict.append(char)

    for char in dict:
        counter += 1
    return counter


def handle_part_2(cache, counter):
    dict = []
    if cache[0] != None:
        dict.append(cache.pop(0))
    for line in cache:
        temp = ""
        for char in line:
            if char in dict[len(dict)-1]:
                temp += char
        dict.append(temp)

    for char in dict.pop():
        counter += 1
    return counter


f = open("./day6/data.txt", "r")
# print(f.read())
counter_part_1 = 0
counter_part_2 = 0
lineArray = f.readlines()
cache = []
for line in lineArray:
    if(line == "\n"):
        # print(line)
        counter_part_1 = handle_part_1(cache, counter_part_1)
        counter_part_2 = handle_part_2(cache, counter_part_2)
        cache = []
    else:
        line = line.strip()
        cache.append(line)
counter_part_1 = handle_part_1(cache, counter_part_1)
counter_part_2 = handle_part_2(cache, counter_part_2)


print(counter_part_1)
print(counter_part_2)
