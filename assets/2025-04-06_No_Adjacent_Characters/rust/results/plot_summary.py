import sys
import matplotlib.pyplot as plt
import numpy as np

DEFAULT_DPI = 150

output_folder = sys.argv[1]

# Static data
categories = ['Simple Array', 'Simple Counter', 'Max Heap', 'Thread Everything', 'Optimized']
data = {
    2:  [13, 10.21, 10.16, 1.87, 1.81],
    3:  [13, 10.85, 10.49, 8.67, 2.75],
    255:[13, 10.94, 10.67, 8.27, 2.71]
}

bar_width = 0.25
x = np.arange(len(categories))  # [0, 1, 2, 3, 4]

def plot_chart(style, filename):
    plt.style.use('dark_background' if style == 'dark' else 'default')
    fig, ax = plt.subplots(figsize=(10, 6))

    offsets = [-bar_width, 0, bar_width]
    colors = ['tab:blue', 'tab:orange', 'tab:green']
    labels = list(data.keys())

    for i, (key, offset, color) in enumerate(zip(labels, offsets, colors)):
        values = data[key]
        ax.bar(x + offset, values, width=bar_width, label=f'{key} Unique Chars', color=color)

    ax.set_xlabel("Algorithm")
    ax.set_ylabel("Aprrox Time (seconds)")
    ax.set_title("1 Billion Characters")
    ax.set_xticks(x)
    ax.set_xticklabels(categories)
    ax.legend()
    ax.grid(axis='y', linewidth=0.3, color='gray', alpha=0.7)

    plt.tight_layout()
    plt.savefig(filename, dpi=DEFAULT_DPI)
    plt.close()
    print(f"{style.title()} chart saved to {filename}")

plot_chart("light", output_folder+ "/summary_light.png")
plot_chart("dark", output_folder+ "/summary_dark.png")

