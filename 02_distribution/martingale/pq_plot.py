import pandas as pd
import matplotlib.pyplot as plt
import scienceplots
import numpy as np

# Import parquet file
df = pd.read_parquet('./martingale.parquet')

# Prepare Data to Plot
n_vec    = df['n']
mean_vec = df['mean']
std_vec  = df['std']
max_vec  = df['max']

# Plot params
pparam = dict(
    xlabel = r'Round',
    ylabel = r'Loss',
    xscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(n_vec, mean_vec, label='Mean')
    ax.legend()
    fig.savefig('martingale_mean.png', dpi=600, bbox_inches='tight')

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(n_vec, std_vec, label='Std')
    ax.legend()
    fig.savefig('martingale_std.png', dpi=600, bbox_inches='tight')

# Plot params
pparam = dict(
    xlabel = r'Round',
    ylabel = r'Loss',
    xscale = 'linear',
    yscale = 'log'
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(n_vec, max_vec, label='Max')
    ax.legend()
    fig.savefig('martingale_max.png', dpi=600, bbox_inches='tight')
