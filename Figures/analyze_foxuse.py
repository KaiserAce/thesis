import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns #type:ignore
from matplotlib.ticker import PercentFormatter

# Parameters
pop_sizes = [20, 50, 100, 200, 500]
colors = sns.color_palette("YlOrBr", len(pop_sizes))

# To store all y-values (fox usage) across population sizes for each f value
combined_fox_usage = []

# Simulation keys
keys = {
    20: ['2A441A0Y', 'R0Y7V4G8', '319XBWAA', 'PIW3D2EV', 'O3IUU5OR', 'YS9FAHNY', 'ZZHE2LPD', 'VN9X5PH9', '1F4D0G0W', 'ODZHZUTU', 'JTSFSQFJ'],
    50: ['LQF6DSA3', '1TZ9OA2Z', '0I0NLCK1', 'VQH0JJDB', '0KE9ZXTC', '2IUKH41E', 'AKXXHKE2', '8DW4HBGF', '7NIDF7VO', 'VT2UGZQY', 'DU6LVVBX'],
    100: ['BH6H2C5J', 'FHKN74I8', '6Y7BWPP9', 'PBRUQUNU', 'C8KAZC7V', 'AXXNB05Y', 'HMNMO4S3', 'IHSRB9GE', 'V9KY8HIO', 'RQ7CNP81', 'GMBNQZCH'],
    200: ['X2L32SQI', '370WWRF7', '1FE3CLLT', 'YYFT4Q9Y', '0O6QKL5C', 'SH91QCBG', 'PJNW119E', 'KY0IYZ1R', 'SDJSE2AA', 'BFR98VMX', 'P97S2CXS'],
    500: ['Y8Y4C9OD', 'ZKJD3ALH', '5L4P7UT0', 'N2OPV52G', 'RUT98DYS', 'Y7J9STBP', 'ENGWEA7E', '6BQI5M1R', 'SPZE8BZO', 'G9888QXC', '913UC8SU']
}

# Plot setup
plt.figure(figsize=(12, 7))
sns.set(style="whitegrid")

# Process and plot for each population size
for idx, pop_size in enumerate(pop_sizes):
    avg_fox_usage = []

    for key in keys[pop_size]:
        filename = f"HDInnov_FoxUsage_{key}_1_7.csv"
        df = pd.read_csv(filename, header=None)
        # Each row: one simulation run; each value: 0 or 1 if agent used fox
        mean_usage_per_run = df.mean(axis=1)  # Mean fox use across agents per run
        mean_over_all_runs = mean_usage_per_run.mean()  # Mean across all runs
        avg_fox_usage.append(mean_over_all_runs)
        combined_fox_usage.append(avg_fox_usage)

    # Get corresponding X values (e.g., pick 11 evenly spaced points from proportion)
    x_values = np.linspace(0.0, 1.0, 11)
    plt.plot(x_values, avg_fox_usage, label=f"N = {pop_size}", color=colors[idx], linewidth=2)

# Calculate average across population sizes for each f value
combined_fox_usage = np.array(combined_fox_usage)
general_trend = np.nanmean(combined_fox_usage, axis=0)

# Plot general trendline
plt.plot(x_values, general_trend, label="General Trend", color="black", linestyle="--", linewidth=2)

# Final plot settings
plt.ylabel("Proportion of Foxes in Population")
plt.xlabel("Power Asymmetry f")
plt.ylim(0.02,0.04)
plt.xticks(x_values)
plt.title("Power Asymmetry vs. Proportion of Foxes (by Population Size)")
plt.legend(title="Population Size")
plt.gca().yaxis.set_major_formatter(PercentFormatter(1.0))
plt.tight_layout()
plt.savefig("Fox Proportion Across Populations.png")
plt.show()