import matplotlib.pyplot as pyplot
import numpy

FILE_NAME = "data.csv"
file_stream = open(FILE_NAME);
algorithms = []
scores = []
# -1 to account for the N value in the first row
tested_algorithms_count = -1
first_line = True
for line in file_stream:
    temp_score = []
    for tokens in line[:-1].split(','):
        if first_line:
            algorithms.append(tokens)
            tested_algorithms_count += 1
        else:
            temp_score.append(int(tokens))
    if first_line:
        first_line = False
    else:
        scores.append(temp_score)

pyplot.style.use('_mpl-gallery')

x = [sample_count[0] for sample_count in scores]
ys = [[performance_score[i + 1] for performance_score in scores] for i in range(tested_algorithms_count)]

fig, ax = pyplot.subplots()
[ax.plot(x, y) for y in  ys]

pyplot.savefig("plot.png", dpi=500)