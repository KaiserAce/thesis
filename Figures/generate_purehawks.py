import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns #type:ignore
from matplotlib.ticker import PercentFormatter

folders = [20, 50, 100, 200, 500]
keys = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
directory = "../Output/population/"

for folder in folders:
    data = []
    for key in keys:
        visit = pd.read_csv(f"{directory}{folder}/ff_{key}/StrategyVisit_0.csv", header=None)
        host = pd.read_csv(f"{directory}{folder}/ff_{key}/StrategyHost_0.csv", header=None)

        visitor_strat = visit.iloc[0, ::3].values
        host_strat = host.iloc[0, ::2].values

        count = sum((visitor_strat > 0.9) & (host_strat > 0.9))
        data.append((count / folder) * 100)

    plt.plot(keys, data, label=f"{folder}")

plt.xlabel("Proportion of pure hawks")
plt.ylabel("f")
plt.legend(title="Population")
plt.title("Pure Hawks Across Populations")

plt.savefig("./population/Pure Hawk Proportion.png")
