import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

dynamic = "./DynamicRankModel/"
static = "./StaticRankModel/"
output = "HDInnov_Output_Data/"
d_output = "Output_HDInnov_Pop-20_Discount-N-0.01_S-0.01_Tremble-0.01_NLS-0-1_SLS-1_SymN-0_SymS-0_CopyProb-0_CopyError-0_ExpProb-0_InnovNoise-0_CoupleFn-Fight_P-0.0-1.0-0.4-0.6-0.7-0.8_Bon-0.6/"
s_output = "Output_HDInnov_Pop-20_Discount-ND-0.01_SD-0.01_Tremble-0.01_NLS-0-1_SLS-1_SymN-0_SymS-0_CopyProb-0_CopyError-0_ExpProb-0_InnovNoise-0_CoupleFn-Fight_P-0.0-1.0-0.4-0.6-0.7-0.8_Bon-0.6/"
dynamic_direc = dynamic + output + d_output
static_direc = static + output + s_output

sn_sr_payoff = np.sort(pd.read_csv(static_direc + 'HDInnov_TotalPayoff_6OCRN5UW_1_7.csv', header=None).iloc[-1][0:21])
sn_dr_payoff = np.sort(pd.read_csv(dynamic_direc + 'HDInnov_TotalPayoff_O9ERYGXL_1_7.csv', header=None).iloc[-1][0:21])
dn_sr_payoff = np.sort(pd.read_csv(static_direc + 'HDInnov_TotalPayoff_EFFHPXPM_1_7.csv', header=None).iloc[-1][0:21])
dn_dr_payoff = np.sort(pd.read_csv(dynamic_direc + 'HDInnov_TotalPayoff_MCCCEGOB_1_7.csv', header=None).iloc[-1][0:21])

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

population = (np.arange(0, 21, 1)) / 20 * 100

norm_sn_sr = (np.cumsum(sn_sr_payoff) / np.sum(sn_sr_payoff)) * 100
norm_sn_dr = (np.cumsum(sn_dr_payoff) / np.sum(sn_dr_payoff)) * 100
norm_dn_sr = (np.cumsum(dn_sr_payoff) / np.sum(dn_sr_payoff)) * 100
norm_dn_dr = (np.cumsum(dn_dr_payoff) / np.sum(dn_dr_payoff)) * 100

color = ['#B8E183', '#4CAC26', '#F2B6DA', '#D01C8A']

fig, ax = plt.subplots(figsize=(6, 6))

ax.plot(population, np.insert(norm_sn_sr, 0, 0), color=color[0])
ax.plot(population, np.insert(norm_sn_dr, 0, 0), color=color[1])
ax.plot(population, np.insert(norm_dn_sr, 0, 0), color=color[2])
ax.plot(population, np.insert(norm_dn_dr, 0, 0), color=color[3])

# Move the y-axis to the right
ax.spines['right'].set_position(('outward', 0))  # Or ('axes', 1) for relative position
ax.spines['left'].set_visible(False)  # Hide the original left spine
ax.spines['top'].set_visible(False)  # Hide the original left spine

# Ensure ticks and labels are also on the right
ax.yaxis.set_ticks_position('right')
ax.yaxis.set_label_position('right')

ax.set_xticks([0, 25, 50, 75, 100])
ax.set_yticks([0, 25, 50, 75, 100])

ax.set_xlabel('Cumulative Population (%)')
ax.set_ylabel('Cumulative Wealth (%)')

plt.savefig("Lorenz Curve.png")
plt.show()
