import os
import json
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from itertools import combinations

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

RUST_EXECUTABLE = "target/release/nauty-colourings"
OUTPUT_DIR = "cycle_path_analysis"
Path(OUTPUT_DIR).mkdir(exist_ok=True)

# Exponential function for fitting
def exponential_func(x, a, b):
    return np.exp(a * x + b)

# ---------- GRAPH GENERATORS ----------
def generate_cycle_graphs(min_n=3, max_n=20):
    graphs = []
    for n in range(min_n, max_n + 1):
        edges = [(i, (i + 1) % n) for i in range(n)]
        graphs.append({
            "name": f"Cycle_{n}",
            "vertices": n,
            "edges": edges
        })
    return graphs

def run_rust_algorithm(graph):
    temp_json = f"{OUTPUT_DIR}/temp_{graph['name']}.json"
    with open(temp_json, "w") as f:
        json.dump(graph, f)
    result = subprocess.run([RUST_EXECUTABLE, temp_json], capture_output=True, text=True)
    os.remove(temp_json)
    if result.returncode != 0:
        raise RuntimeError(f"Rust execution failed for {graph['name']}: {result.stderr}")
    return json.loads(result.stdout)

def plot_semilog_exponential_fit(df, label):
    vertices = np.array(df['vertices'])
    runtimes = np.array(df['runtime_ms'])
    params, _ = curve_fit(exponential_func, vertices, runtimes, p0=(0.7, -7))
    a, b = params
    exp_fit = exponential_func(vertices, a, b)

    plt.figure(figsize=(8, 6))
    plt.semilogy(vertices, runtimes, 'o-', label='Measured')
    plt.semilogy(vertices, exp_fit, 'r--', label=f'Fit: $e^{{{a:.2f}n + {b:.2f}}}$')
    plt.xlabel("Number of Vertices (n)")
    plt.ylabel("Runtime (ms)")
    plt.title(f"Semilog Exponential Fit for {label}")
    plt.legend()
    plt.grid(True, which="both")
    plt.savefig(f"{OUTPUT_DIR}/{label}_semilog_exp_fit.png")
    plt.close()

def analyze_graph_family(graph_generator, label):
    results = []
    for graph in graph_generator():
        try:
            output = run_rust_algorithm(graph)
            results.append({
                "name": graph["name"],
                "vertices": graph["vertices"],
                "edges": len(graph["edges"]),
                "orbits": output["orbits"],
                "runtime_ms": output["runtime_ms"]
            })
        except Exception as e:
            print(f"Error for {graph['name']}: {e}")
    df = pd.DataFrame(results)
    df.to_csv(f"{OUTPUT_DIR}/{label}_results.csv", index=False)
    plot_semilog_exponential_fit(df, label)

def main():
    analyze_graph_family(generate_cycle_graphs, "CycleGraphs")

if __name__ == "__main__":
    main()
