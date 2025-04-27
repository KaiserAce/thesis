import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

dynamic = "./DynamicRankModel/"
static = "./StaticRankModel/"
output = "HDInnov_Output_Data/"
d_output = "Output_HDInnov_Pop-20_Discount-N-0.01_S-0.01_Tremble-0.01_NLS-0-1_SLS-1_SymN-0_SymS-0_CopyProb-0_CopyError-0_ExpProb-0_InnovNoise-0_CoupleFn-Fight_P-0.0-1.0-0.4-0.6-0.7-0.8_Bon-0.6/"
s_output = "Output_HDInnov_Pop-20_Discount-ND-0.01_SD-0.01_Tremble-0.01_NLS-0-1_SLS-1_SymN-0_SymS-0_CopyProb-0_CopyError-0_ExpProb-0_InnovNoise-0_CoupleFn-Fight_P-0.0-1.0-0.4-0.6-0.7-0.8_Bon-0.6/"
dynamic_direc = dynamic + output + d_output
static_direc = static + output + s_output

staticNet_staticRank = pd.read_csv(static_direc + 'HDInnov_EvoStats_6OCRN5UW_1_7.csv', header=None)
dynamicNet_staticRank = pd.read_csv(static_direc + 'HDInnov_EvoStats_EFFHPXPM_1_7.csv', header=None)

staticNet_dynamicRank = pd.read_csv(dynamic_direc + 'HDInnov_EvoStats_O9ERYGXL_1_7.csv', header=None)
dynamicNet_dynamicRank = pd.read_csv(dynamic_direc + 'HDInnov_EvoStats_MCCCEGOB_1_7.csv', header=None)

sn_sr_payoff = pd.read_csv(static_direc + 'HDInnov_TotalPayoff_6OCRN5UW_1_7.csv', header=None)
sn_dr_payoff = pd.read_csv(dynamic_direc + 'HDInnov_TotalPayoff_O9ERYGXL_1_7.csv', header=None)
dn_sr_payoff = pd.read_csv(static_direc + 'HDInnov_TotalPayoff_EFFHPXPM_1_7.csv', header=None)
dn_dr_payoff = pd.read_csv(dynamic_direc + 'HDInnov_TotalPayoff_MCCCEGOB_1_7.csv', header=None)

pd.set_option("display.max_columns", None)
pd.set_option("display.max_rows", None)

# pie_labels = ['Hawk-Hawk', 'Hawk-Dove', 'Dove-Hawk', 'Dove-Dove']
# pie_colors = ['#DF2928', '#FB916F', '#9FC9DE', '#2F7EB9']

fig = plt.figure(figsize=(8, 6))

# ax1 = plt.subplot2grid((2, 4), (0, 0))
# ax2 = plt.subplot2grid((2, 4), (0, 1))
# ax3 = plt.subplot2grid((2, 4), (0, 2))
# ax4 = plt.subplot2grid((2, 4), (0, 3))
# ax5 = plt.subplot2grid((2, 4), (1, 0), colspan=4)

# w1, _, _ = ax1.pie(np.array(staticNet_staticRank.iloc[14][1:5]), colors=pie_colors, autopct='%1.1f%%', startangle=90)

# w2, _, _ = ax2.pie(np.array(staticNet_dynamicRank.iloc[33][1:5]), colors=pie_colors, autopct='%1.1f%%', startangle=90)

# w3, _, _ = ax3.pie(np.array(dynamicNet_staticRank.iloc[63][1:5]), colors=pie_colors, autopct='%1.1f%%', startangle=90)

# w4, _, _ = ax4.pie(np.array(dynamicNet_dynamicRank.iloc[21][1:5]), colors=pie_colors, autopct='%1.1f%%', startangle=90)

# fig.legend(w1, pie_labels, loc="upper center", ncol=len(pie_labels))

bar_data = [np.sum(sn_sr_payoff.iloc[-1][0:21]), 
            np.sum(sn_dr_payoff.iloc[-1][0:21]), 
            np.sum(dn_sr_payoff.iloc[-1][0:21]), 
            np.sum(dn_dr_payoff.iloc[-1][0:21])]
bar_labels = ['Static network,\n static rank', 
              'Static network,\n dynamic rank', 
              'Dynamic network,\n static rank', 
              'Dynamic network,\n dynamic rank']

plt.bar(bar_labels, bar_data, color=['#B8E183', '#4CAC26', '#F2B6DA', '#D01C8A'])
# plt.set_ylabel('Total Payoffs')
plt.yticks(rotation=90)
plt.xticks(rotation=45)

plt.tight_layout()
plt.savefig("4Bar Payoffs.png")
plt.show()
