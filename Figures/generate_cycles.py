import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

directory = "../Output/dynamicrank_dynamicnet/ff_0.4/"


out_scores = pd.read_csv(f"{directory}OutScore_0.csv", header=None)
in_strength = pd.read_csv(f"{directory}NetSTD_0.csv", header=None)
strategy_host = pd.read_csv(f"{directory}StrategyHost_0.csv", header=None)
strategy_visit = pd.read_csv(f"{directory}StrategyVisit_0.csv", header=None)
morality = pd.read_csv(f"{directory}Morality_0.csv", header=None)
fox_usage = pd.read_csv(f"{directory}FoxUsage_0.csv", header=None)


num_iterations = len(morality)

agent = 10

ranks = np.zeros(num_iterations)
weights = np.zeros(num_iterations)
host_strategy = np.zeros(num_iterations)
visit_strategy = np.zeros(num_iterations)
morals = np.zeros(num_iterations)
fox_use = np.zeros(num_iterations)

for i in range(num_iterations):
    ranks[i] = out_scores.iloc[i][agent]
    weights[i] = in_strength.iloc[i][agent]
    host_strategy[i] = strategy_host.iloc[i][agent*2]
    visit_strategy[i] = strategy_visit.iloc[i][agent*2]
    morals[i] = morality.iloc[i][agent]
    fox_use[i] = fox_usage.iloc[i][agent]

ranks_normalized = (ranks - np.min(ranks)) / (np.max(ranks) - np.min(ranks))
weights_normalized = (weights - np.min(weights)) / (np.max(weights) - np.min(weights))
morals_normalize = (morals - np.min(morals)) / (np.max(morals) - np.min(morals))
Time = np.arange(1, num_iterations+1)

plt.figure(figsize=(12, 6))
plt.plot(Time, ranks_normalized, color='black', linestyle='dashed', label="Rank")
plt.plot(Time, weights_normalized, color='green', linestyle='dashed', label="In-strength")
plt.plot(Time, host_strategy, color='orange', label='Host Strategy')
plt.plot(Time, visit_strategy, color='magenta', label='Visit Strategy')
plt.plot(Time, morals, color='blue', linestyle='dotted', label='Morality')
plt.plot(Time, fox_use, color='red', linestyle='dotted',  label='Fox Usage')

plt.yticks([0, 1], ["Minimum", "Maximum"])


plt.legend(loc="upper center", bbox_to_anchor=(0.5, -0.15), ncol=2)

# plt.xlim(25000, 29000) 
plt.xlim(0,num_iterations)
plt.ylim(-0.05, 1.05)
plt.xlabel('Time')

plt.tight_layout()
plt.savefig("./dynamicrank_dynamicnet/Agent All Cycles.png")
