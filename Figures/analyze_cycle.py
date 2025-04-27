import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

# directory = './HDInnov_Output_Data/Output_HDInnov_Pop-20_Discount-N-0.01_S-0.01_Tremble-0.01_NLS-1_SLS-1_SymN-0_SymS-0_CopyProb-0_CopyError-0_ExpProb-0_InnovNoise-0_CoupleFn-Fight_P-0.0-1.0-0.4-0.6-0.7-0.8_Bon-0.4'
out_scores = pd.read_csv('HDInnov_OutScore_TBN6FRD9_1_7.csv', header=None)
in_strength = pd.read_csv('HDInnov_NetSTD_TBN6FRD9_1_7.csv', header=None)
strategy = pd.read_csv('HDInnov_OutFS_TBN6FRD9_1_7.csv', header=None)
morality = pd.read_csv('HDInnov_Morality_TBN6FRD9_1_7.csv', header=None)
fox_usage = pd.read_csv('HDInnov_FoxUsage_TBN6FRD9_1_7.csv', header=None)

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

num_rows = len(morality) 
num_iterations = num_rows // 20 

num_iterations = len(morality) // 20 

ranks = np.zeros(num_iterations)
weights = np.zeros(num_iterations)
host_strategy = np.zeros(num_iterations)
visit_strategy = np.zeros(num_iterations)
morals = np.zeros(num_iterations)
fox_use = np.zeros(num_iterations)
i = 0
for i in range(num_iterations):
    ranks[i] = out_scores.iloc[(i*20 + 6)][0]
    weights[i] = in_strength.iloc[(i)][6]
    host_strategy[i] = strategy.iloc[(i*20 + 6)][1]
    visit_strategy[i] = strategy.iloc[(i*20) + 6][0]
    morals[i] = morality.iloc[(i)][6]
    fox_use[i] = fox_usage.iloc[(i)][6]
    i += 1

ranks_normalized = (ranks - np.min(ranks)) / (np.max(ranks) - np.min(ranks))
weights_normalized = (weights - np.min(weights)) / (np.max(weights) - np.min(weights))
morals_normalize = (morals - np.min(morals)) / (np.max(morals) - np.min(morals))
fox_normalize = (fox_use - np.min(fox_use)) / (np.max(fox_use) - np.min(fox_use))
Time = np.arange(1, num_iterations+1)

host_strategy = np.clip(host_strategy, 0, 1)
visit_strategy = np.clip(visit_strategy, 0, 1)

plt.figure(figsize=(12, 6))
plt.plot(Time, ranks_normalized, color='black', linestyle='dashed', label="Rank")
plt.plot(Time, weights_normalized, color='green', linestyle='dashed', label="In-strength")
plt.plot(Time, host_strategy, color='orange', label='Host Strategy')
plt.plot(Time, visit_strategy, color='magenta', label='Visit Strategy')
plt.plot(Time, morals, color='blue', linestyle='dotted', label='Morality')
plt.plot(Time, fox_normalize, color='red', linestyle='dotted',  label='Fox Usage')

plt.yticks([0, 1], ["Minimum", "Maximum"])


plt.legend(loc="upper center", bbox_to_anchor=(0.5, -0.15), ncol=2)

plt.xlim(25000, 29000) 
# plt.xlim(0,num_iterations)
plt.ylim(-0.05, 1.05)
plt.xlabel('Time')

plt.tight_layout()
plt.savefig("Agent All Cycles.png")
plt.show()
