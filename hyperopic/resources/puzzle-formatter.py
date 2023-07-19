fsrc, ftar = open('three-puzzles'), open('tp2', 'w')
content = (line.strip() for line in fsrc if line != '\n')
for (line, i) in zip(content, range(1000000)):
    imod3 = i % 3
    if imod3 == 0:
        continue
    elif imod3 == 1:
        ftar.write(line)
        ftar.write('$$$$')
    else:
        ftar.write(line)
        ftar.write('\n')

fsrc.close()
ftar.close()
