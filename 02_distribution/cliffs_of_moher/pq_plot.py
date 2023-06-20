import pandas as pd
import matplotlib.pyplot as plt
import scienceplots
import numpy as np

# Import parquet file
df = pd.read_parquet('./cliff.parquet')

# Prepare Data to Plot
safe = df['safe']
quick = df['quick']

safe_mean = np.mean(safe)
quick_mean = np.mean(quick)

# Plot params
pparam = dict(
    xlabel = r'Reward',
    ylabel = r'Density',
    xscale = 'linear',
    yscale = 'linear',
    xlim   = (-1.0, 1.0)
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.hist(safe, bins=100, density=True, histtype='step', color='b', alpha=0.5, label=f'Safe (mean={safe_mean:.2f})')
    ax.hist(quick, bins=100, density=True, histtype='step', color='r', alpha=0.5, label=f'Quick (mean={quick_mean:.2f})')
    ax.legend()
    fig.savefig('hist.png', dpi=600, bbox_inches='tight')
