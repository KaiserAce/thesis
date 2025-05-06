import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches

direc = "../Output/"
folders = ["staticrank_staticnet", 'staticrank_dynamicnet', 'dynamicrank_dynamicnet', 'dynamicrank_staticnet']

fig, ax = plt.subplots(figsize=(8, 6))

for path in folders:
    file = pd.read_csv(f"{direc}{path}/ff_0.6/TotalPayoff_0.csv", header=None) 
    unsorted = file.iloc[-1][0:21]
    values = np.cumsum(np.sort(unsorted))
    sum = values[-1]
    rel_val = []
    for value in values:
        rel_val.append(value / sum)
    ax.plot(np.arange(len(values)), rel_val, label=path)

plt.tight_layout()
plt.savefig("./payoffs/CumulativePayoffs.png")
