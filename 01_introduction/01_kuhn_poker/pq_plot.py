import pandas as pd
import matplotlib.pyplot as plt
import scienceplots
import numpy as np
import argparse

# Parse arguments
parser = argparse.ArgumentParser(description='Plot parquet file.')
parser.add_argument('round', type=int)
args = parser.parse_args()
round = args.round

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
    ylim   = (0, 0.88)
)

x, y = np.unique(x, return_counts=True)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    #ax.hist(x, bins=24, density=True, label='Histogram')
    ax.bar(x, y / y.sum(), label=rf'$T={round}$')
    ax.legend()
    fig.savefig('hist.png', dpi=600, bbox_inches='tight')
