arr = []

for i in range(1, 100):
    if i % 2 == 1:
        arr.append(i)
print(' '.join([str(e) for e in arr[10:]]))