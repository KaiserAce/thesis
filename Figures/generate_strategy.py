import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches

rows, cols = 11, 20
keys = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
folders = ["staticrank_staticnet", 'staticrank_dynamicnet', 'dynamicrank_dynamicnet', 'dynamicrank_staticnet']

fig, ax = plt.subplots(figsize=(8, 6))

for path in folders:
    directory = f"../Output/{path}/"
    os.makedirs(path, exist_ok=True)
    for index, key in enumerate(keys):
        rank = pd.read_csv(f"{directory}ff_{key}/OutScore_0.csv")
        Vstrat = pd.read_csv(f"{directory}ff_{key}/StrategyVisit_0.csv")
        Hstrat = pd.read_csv(f"{directory}ff_{key}/StrategyHost_0.csv")

        ranks = rank.iloc[-1, :].values
        visitor_strat = Vstrat.iloc[-1, :].values
        host_strat = Hstrat.iloc[-1, :].values

        agent_visitor = visitor_strat.reshape(-1, 3)
        agent_host = host_strat.reshape(-1, 2)

        for entry in range(cols):
            outer_val = agent_visitor[entry, 0]
            inner_val = agent_host[entry, 0]
            fox_val = agent_visitor[entry, 2]

            alpha = 1.0
            y_pos = 1.1 * index

            color_outer = '#705259' if outer_val > 0.8 else '#167d9c'
            color_inner = '#705259'  if inner_val > 0.8 else '#167d9c'

            base_color = '#efa07d'
            rgba_color = mcolors.to_rgba(base_color, fox_val)

            circle1 = mpatches.Circle((ranks[entry], y_pos), 0.4, color=color_outer, alpha=alpha)
            circle2 = mpatches.Circle((ranks[entry], y_pos), 0.2, color=color_inner, alpha=alpha)
            circle3 = mpatches.Circle((ranks[entry], y_pos), 0.05, color=rgba_color, alpha=fox_val)

            ax.add_artist(circle1)
            ax.add_artist(circle2)
            ax.add_artist(circle3)

    ax.set_xlim(0.5, cols + 0.5)
    ax.set_ylim(-0.5, rows + 0.5)

# Set axis ticks and labels
    ax.set_xticks(range(1, cols + 1))

# Explicitly calculate y-ticks and labels
    y_ticks = np.linspace(0, rows, rows)  # Create evenly spaced ticks
    y_labels = [f"{tick/rows:.1f}" for tick in y_ticks]  # Labels range from 0.0 to 1.0
    ax.set_yticks(y_ticks)
    ax.set_yticklabels(y_labels)

# Add labels, title, and formatting
    ax.set_xlabel("Rank")
    ax.set_ylabel("f")
    ax.set_aspect('equal')

    plt.title(path)
    plt.savefig(f"./{path}/{path}.png")
