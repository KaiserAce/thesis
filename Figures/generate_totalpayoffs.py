import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

direc = "../Output/"
folders = ["staticrank_staticnet", 'staticrank_dynamicnet', 'dynamicrank_dynamicnet', 'dynamicrank_staticnet']

values = []

fig = plt.figure(figsize=(8, 6))

for path in folders:
    file = pd.read_csv(f"{direc}{path}/ff_0.6/TotalPayoff_0.csv", header=None) 
    values.append(np.sum(file.iloc[-1][0:21]))

plt.bar(folders, values, color=['#B8E183', '#4CAC26', '#F2B6DA', '#D01C8A'])

plt.tight_layout()
plt.savefig("./payoffs/TotalPayoffs.png")
