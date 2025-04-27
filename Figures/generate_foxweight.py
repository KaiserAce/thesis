import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches
import seaborn as sns

rows, cols = 11, 20
keys = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
folders = ["staticrank_staticnet", 'staticrank_dynamicnet', 'dynamicrank_dynamicnet', 'dynamicrank_staticnet']


for path in folders:
    directory = f"../Output/{path}/"
    os.makedirs(path, exist_ok=True)
    for index, key in enumerate(keys):
        strat_file = pd.read_csv(f"{directory}ff_{key}/StrategyVisit_0.csv")
        ranks_file = pd.read_csv(f"{directory}ff_{key}/OutScore_0.csv")

        fox_weights = strat_file.iloc[:, 2::3].to_numpy().T 
        ranks = ranks_file.iloc[-1, :].to_numpy()

        sorted_ranks = np.argsort(ranks)
        sorted_fox_weights = fox_weights[sorted_ranks, :]

        plt.figure(figsize=(16, 8))
        sns.heatmap(sorted_fox_weights, cmap="magma_r", vmin=0, vmax=1,
                    cbar_kws={'label': 'Fox Strategy Weight'},
                    yticklabels=False)  # Disable default y-tick labels

        plt.yticks(ticks=np.arange(fox_weights.shape[0]) + 0.5,
                labels=[str(i + 1) for i in range(fox_weights.shape[0])],
                rotation=0)

        plt.title(f"Fox Strategy Weight Over Time (Sorted by Final Rank)\nPower Asymmetry f: {key}")
        plt.xlabel("Time Step")
        plt.ylabel("Agents")
        plt.tight_layout()

        plt.savefig(f"./{path}/HeatMap{key}.png", dpi=300)
        plt.close()
