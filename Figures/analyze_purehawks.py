import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns  # type: ignore

# Parameters
proportion_hawks = np.linspace(0, 0.25, 100)  # X-axis: proportion of pure hawks
pop_sizes = [20, 50, 100, 200, 500]  # Different population sizes
colors = sns.color_palette("YlOrBr", len(pop_sizes))  # Matching the gradient

# Keys from 0.0 to 1.0
keys = {
    20: ['2A441A0Y', 'R0Y7V4G8', '319XBWAA', 'PIW3D2EV', 'O3IUU5OR', 'YS9FAHNY', 'ZZHE2LPD', 'VN9X5PH9', '1F4D0G0W', 'ODZHZUTU', 'JTSFSQFJ'],
    50: ['LQF6DSA3', '1TZ9OA2Z', '0I0NLCK1', 'VQH0JJDB', '0KE9ZXTC', '2IUKH41E', 'AKXXHKE2', '8DW4HBGF', '7NIDF7VO', 'VT2UGZQY', 'DU6LVVBX'],
    100: ['BH6H2C5J', 'FHKN74I8', '6Y7BWPP9', 'PBRUQUNU', 'C8KAZC7V', 'AXXNB05Y', 'HMNMO4S3', 'IHSRB9GE', 'V9KY8HIO', 'RQ7CNP81', 'GMBNQZCH'],
    200: ['X2L32SQI', '370WWRF7', '1FE3CLLT', 'YYFT4Q9Y', '0O6QKL5C', 'SH91QCBG', 'PJNW119E', 'KY0IYZ1R', 'SDJSE2AA', 'BFR98VMX', 'P97S2CXS'],
    500: ['Y8Y4C9OD', 'ZKJD3ALH', '5L4P7UT0', 'N2OPV52G', 'RUT98DYS', 'Y7J9STBP', 'ENGWEA7E', '6BQI5M1R', 'SPZE8BZO', 'G9888QXC', '913UC8SU']
}

# Function to calculate percentages of pure hawks for different population sizes
def calculate_pure_hawks_percentage(keys, total_population):
    count_pure_hawks = []
    
    for key in keys:
        Vstrat = pd.read_csv(f"HDInnov_StrategyVisit_{key}_1_7.csv")
        Hstrat = pd.read_csv(f"HDInnov_StrategyHost_{key}_1_7.csv")
        
        visitor_strat = Vstrat.iloc[-1, :].values
        host_strat = Hstrat.iloc[-1, :].values
        agent_visit_strat = visitor_strat.reshape(-1, 2)
        agent_host_strat = host_strat.reshape(-1, 2)
        
        # Count pure hawks (both values > 0.9)
        count = sum((agent_visit_strat[:, 0] > 0.9) & (agent_host_strat[:, 0] > 0.9))
        count_pure_hawks.append(count)
    
    # Convert counts to percentages
    percentage_pure_hawks = [(count / total_population) * 100 for count in count_pure_hawks]
    return percentage_pure_hawks

# Loop through population sizes to calculate percentages and plot
for i, pop_size in enumerate(pop_sizes):
    total_population = pop_size
    population_keys = keys[pop_size]
    f_values = np.linspace(0, 1, 11)
    # Calculate the percentage of pure hawks for the current population size
    percentage = calculate_pure_hawks_percentage(population_keys, total_population)
    
    # Print the results for each population size
    print(f"Percentage of pure hawks per simulation (Population {pop_size}):", percentage)

    # Plot the results
    plt.plot(percentage, f_values, color=colors[i], label=f"{pop_size}")

# Labels and formatting
plt.xlabel("Proportion of pure hawks")
plt.ylabel("f")
plt.legend(title="Population")
plt.title("Pure Hawks Across Populations")
plt.ylim(-0.1, 1.1)
plt.xticks([0, 10, 20], ["0%", "10%", "20%"])

# Optionally, you can also set custom labels (here, formatted as strings)
labels = [f"{tick:.1f}" for tick in f_values]

plt.yticks(f_values, labels)

plt.savefig("Pure Hawk Proportion.png")
# Display plot
plt.show()