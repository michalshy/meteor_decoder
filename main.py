# validation scripts

import matplotlib.pyplot as plt
import csv

i_vals, q_vals = [], []
with open('filtered_sample.csv') as f:
    for row in csv.reader(f):
        i_vals.append(float(row[0]))
        q_vals.append(float(row[1]))

plt.scatter(i_vals, q_vals, s=1, alpha=0.3)
plt.xlabel('I')
plt.ylabel('Q')
plt.title('Constellation diagram')
plt.axis('equal')
plt.savefig('constellation.png', dpi=150)
plt.show()