import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches

def python_array_to_latex_table(data, caption="", label="", alignment="c"):

    num_cols = len(data[0]) if data else 0
    if isinstance(alignment, str) and len(alignment) == 1:
        col_format = "|" + "|".join(alignment * num_cols) + "|"
    elif isinstance(alignment, str) and len(alignment) == num_cols:
        col_format = "|" + "|".join(alignment) + "|"
    else:
        col_format = "|" + "c|" * num_cols
        if num_cols > 0:
            col_format = col_format[:-1] + "|"

    latex_string = "\\begin{table}[H]\n"
    latex_string += "    \\centering\n"
    if caption:
        latex_string += "    \\caption{" + caption + "}\n"
    if label:
        latex_string += "    \\label{" + label + "}\n"
    latex_string += "    \\begin{tabular}{" + col_format + "}\n"
    latex_string += "        \\hline\n"

    for row in data:
        formatted_row = []
        for item in row:
            if isinstance(item, (int, float)):
                formatted_row.append(f"{item:.2f}")  # Format to two decimal places
            else:
                formatted_row.append(str(item))      # Keep non-numeric items as strings
        row_str = " & ".join(formatted_row) + " \\\\\n"
        latex_string += "        " + row_str
        latex_string += "        \\hline\n"

    latex_string += "    \\end{tabular}\n"
    latex_string += "\\end{table}\n"
    return latex_string


keys = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
folders = ["staticrank_staticnet", 'staticrank_dynamicnet', 'dynamicrank_dynamicnet', 'dynamicrank_staticnet']

for path in folders:
    directory = f"../Output/{path}/"
    os.makedirs(path, exist_ok=True)
    matrix = [[0.0 for _ in range(12)] for _ in range(21)]
    for index, key in enumerate(keys):
        ranks_file = pd.read_csv(f"{directory}ff_{key}/OutScore_0.csv")
        strat_file = pd.read_csv(f"{directory}ff_{key}/StrategyVisit_0.csv")

        fox_weights = strat_file.iloc[-1, 2::3].to_numpy().T 
        ranks = ranks_file.iloc[-1, :].to_numpy()

        for i in range(len(keys)):
            matrix[0][i+1] = keys[i]

        for i in ranks:
            matrix[i][0] = i

        for i in range(len(ranks)):
            matrix[ranks[i]][index + 1] = fox_weights[i]

    latex_code = python_array_to_latex_table(matrix)

    with open(f"./{path}/fox_weight_table.txt", "w") as f:
        f.write(latex_code)

