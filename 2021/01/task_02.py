
with open("input2.txt", "r") as f:
    cnt = 0
    previous_depths = [-1,-1,-1]
    previous_depths_index = 0
    while True:
        line_data = f.readline()
        if len(line_data.strip()) == 0:
            break
        
        depth = int(line_data.strip())
        if previous_depths[previous_depths_index] != -1 and depth > previous_depths[previous_depths_index]:
            cnt += 1
        previous_depths[previous_depths_index] = depth
        previous_depths_index = (previous_depths_index + 1) % 3


print(cnt)
