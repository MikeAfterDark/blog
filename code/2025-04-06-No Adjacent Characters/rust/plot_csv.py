import sys
import pandas as pd
import matplotlib.pyplot as plt
import os

DEFAULT_DPI = 150

def usage():
    print("Usage: python plot_csv_multi.py <x_col> <y1_col> <y2_col> <title> <output_basename> <csv_file1> [csv_file2 ...]")
    sys.exit(1)

if len(sys.argv) < 7:
    usage()

x_col     = sys.argv[1]
y1_col    = sys.argv[2]
y2_col    = sys.argv[3]
title     = sys.argv[4]
basename  = sys.argv[5]
csv_files = sys.argv[6:]

timescale = "nanoseconds"
def plot_chart(style, output_file, color_mode):
    plt.style.use("dark_background" if style == "dark" else "default")
    plt.figure(figsize=(10, 6))

    for i, file in enumerate(csv_files):
        label_base = os.path.splitext(os.path.basename(file))[0]
        df = pd.read_csv(file)
        
        # if doing this then make sure to also change timescale
        # df[y1_col] = df[y1_col] / 1000
        # df[y2_col] = df[y2_col] / 1000

        # Plot first line
        plt.plot(df[x_col], df[y1_col], label=f"{label_base} - {y1_col}")
        plt.annotate(f"({df['success_runs'].iloc[-1]})",
                     xy=(df[x_col].iloc[-1], df[y1_col].iloc[-1]),
                     xytext=(2, -3),
                     textcoords="offset points",
                     fontsize=8,
                     color="gray")

        # Plot second line
        plt.plot(df[x_col], df[y2_col], label=f"{label_base} - {y2_col}")
        plt.annotate(f"({df['fail_runs'].iloc[-1]})",
                     xy=(df[x_col].iloc[-1], df[y2_col].iloc[-1]),
                    xytext=(2, -3),
                     textcoords="offset points",
                     fontsize=8,
                     color="gray")

    plt.xlabel("String Length (count)")
    plt.ylabel("Time (" + timescale + ")")
    plt.title(title)
    plt.legend()
    plt.grid(axis='y', linewidth=0.3, color='gray', alpha=0.7)
    plt.tight_layout()
    plt.savefig(output_file, dpi=DEFAULT_DPI)
    plt.close()
    print(f"{style.title()} chart saved to {output_file}")

plot_chart("light", f"{basename}_light.png", "light")
plot_chart("dark", f"{basename}_dark.png", "dark")
