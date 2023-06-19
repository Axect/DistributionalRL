import pandas as pd
import matplotlib.pyplot as plt
import scienceplots
import numpy as np

# Import parquet file
df = pd.read_parquet('result.parquet')

# Prepare Data to Plot
x = df['win']

# Plot params
pparam = dict(
    xlabel = r'Winnings',
    ylabel = r'Probability',
    xscale = 'linear',
    yscale = 'linear',
    xlim   = (-12, 12),
    ylim   = (0, 0.35)
)

x, y = np.unique(x, return_counts=True)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    #ax.hist(x, bins=24, density=True, label='Histogram')
    ax.bar(x, y / y.sum())
    fig.savefig('hist.png', dpi=600, bbox_inches='tight')
