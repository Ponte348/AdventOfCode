# open input.txt

with open('input.txt') as f:
    lines = f.readlines()
    lines = [line.strip() for line in lines]
    counter=0
    pastline=9999
    for line in lines:
        if int(line) > pastline:
            counter+=1
        pastline=int(line)
        
print(counter)