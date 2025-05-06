import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns #type:ignore
from matplotlib.ticker import PercentFormatter

folders = [20, 50, 100, 200, 500]
keys = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
directory = "../Output/population/"

gen_data = []

for folder in folders:
    data = []
    for key in keys:
        df = pd.read_csv(f"{directory}{folder}/ff_{key}/FoxUsage_0.csv")
        raw = np.mean(df.iloc[1: -1, :])
        entry = raw / 100000
        data.append(entry)
    gen_data.append(data)
    x_values = np.linspace(0.0, 1.0, 11)
    plt.plot(x_values, data, label=f"N = {folder}", linewidth=2)

general_trend = np.nanmean(gen_data, axis=0)

x_values = np.linspace(0.0, 1.0, 11)
plt.plot(x_values, general_trend, label="General Trend", color="black", linestyle="--", linewidth=2)

# Final plot settings
plt.ylabel("Proportion of Foxes in Population")
plt.xlabel("Power Asymmetry f")
plt.xticks(x_values)
plt.title("Power Asymmetry vs. Proportion of Foxes (by Population Size)")
plt.legend(title="Population Size")
plt.gca().yaxis.set_major_formatter(PercentFormatter(1.0))
plt.tight_layout()
plt.savefig("./population/Fox Proportion Across Populations.png")
