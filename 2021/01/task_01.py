
with open("input.txt", "r") as f:
    cnt = 0
    previous_depth = None
    while True:
        line_data = f.readline()
        if len(line_data.strip()) == 0:
            break
        depth = int(line_data.strip())
        if previous_depth is not None:
            if depth > previous_depth:
                cnt += 1
        previous_depth = depth

print(cnt)
