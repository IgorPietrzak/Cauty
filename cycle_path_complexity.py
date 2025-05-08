import os
import json
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import uuid
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit
from math import factorial

RUST_EXECUTABLE = "target/release/nauty-colourings"
OUTPUT_DIR = "cycle_path_analysis"
FIT_MODEL = "factorial_exponential"  MAX_N = 25 
Path(OUTPUT_DIR).mkdir(exist_ok=True)


def exponential_func(x, a, b):
    return np.exp(a * x + b)

def log_exponential_func(x, a, b):
    return a * x + b

def factorial_exponential_func(x, a, b):
    x = np.array(x, dtype=float)
    factorials = np.array([factorial(int(n)) for n in x])
    return a * factorials * np.exp(b * x)

def log_factorial_exponential_func(x, a, b):
    x = np.array(x, dtype=float)
    log_factorials = np.array([sum(np.log(i) for i in range(1, int(n) + 1)) for n in x])
    return np.log(a) + log_factorials + b * x

def generate_cycle_graphs(min_n=3, max_n=MAX_N):
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
    temp_json = f"{OUTPUT_DIR}/temp_{graph['name']}_{uuid.uuid4()}.json"
    try:
        with open(temp_json, "w") as f:
            json.dump(graph, f)
        result = subprocess.run(
            [RUST_EXECUTABLE, temp_json],
            capture_output=True,
            text=True,
            timeout=60
        )
        if result.returncode != 0:
            raise RuntimeError(f"Rust execution failed for {graph['name']}: {result.stderr}")
        try:
            output = json.loads(result.stdout)
            if not isinstance(output, dict) or "orbits" not in output or "runtime_ms" not in output:
                raise ValueError(f" Aragón output format from Rust for {graph['name']}")
            return output
        except json.JSONDecodeError as e:
            raise ValueError(f"Failed to parse Rust output for {graph['name']}: {e}")
    finally:
        if os.path.exists(temp_json):
            try:
                os.remove(temp_json)
            except OSError as e:
                print(f"Warning: Failed to remove {temp_json}: {e}")

def plot_fits(df, label, exp_params=None, fact_exp_params=None):
    vertices = np.array(df['vertices'])
    runtimes = np.array(df['runtime_ms'])

    plt.figure(figsize=(10, 6))
    plt.plot(vertices, runtimes, 'o-', label='Measured', color='blue')

    if exp_params is not None:
        a, b = exp_params
        exp_fit = exponential_func(vertices, a, b)
        plt.plot(vertices, exp_fit, 'r--', label=f'Exponential: $e^{{{a:.2f}n + {b:.2f}}}$')

    if fact_exp_params is not None:
        a, b = fact_exp_params
        fact_exp_fit = factorial_exponential_func(vertices, a, b)
        plt.plot(vertices, fact_exp_fit, 'g-.', label=f'Factorial-Exp: ${a:.2e} \\cdot n! \\cdot e^{{{b:.2f}n}}$')

    plt.xlabel("Number of Vertices (n)")
    plt.ylabel("Runtime (ms)")
    plt.title(f"Runtime Fit for {label}")
    plt.legend()
    plt.grid(True)
    plt.yscale('log')  # Log scale for better visualization
    plt.savefig(f"{OUTPUT_DIR}/{label}_fit.png")
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
            continue

    if not results:
        print(f"No results collected for {label}. Skipping analysis.")
        return

    df = pd.DataFrame(results)
    df.to_csv(f"{OUTPUT_DIR}/{label}_results.csv", index=False)

    vertices = np.array(df['vertices'])
    runtimes = np.array(df['runtime_ms'])
    log_runtimes = np.log(np.maximum(runtimes, 0.1))  # Avoid log(0)

    exp_params = None
    fact_exp_params = None

    # Fit exponential model
    try:
        exp_params, _ = curve_fit(log_exponential_func, vertices, log_runtimes, p0=[0.1, 0])
        a, b = exp_params
        print(f"✅ {label} (Exponential): Estimated complexity = O(e^({a:.2f}n + {b:.2f}))")
    except Exception as e:
        print(f"Failed to fit exponential model for {label}: {e}")

    if FIT_MODEL == "factorial_exponential":
        try:
            fact_exp_params, _ = curve_fit(
                log_factorial_exponential_func, vertices, log_runtimes, p0=[1e-6, 0]
            )
            a, b = fact_exp_params
            print(f"✅ {label} (Factorial-Exponential): Estimated complexity = O({a:.2e} * n! * e^({b:.2f}n))")
        except Exception as e:
            print(f"Failed to fit factorial-exponential model for {label}: {e}")

    plot_fits(df, label, exp_params, fact_exp_params)

def main():
    if not os.path.isfile(RUST_EXECUTABLE):
        print(f"Error: Rust executable '{RUST_EXECUTABLE}' not found.")
        return
    analyze_graph_family(generate_cycle_graphs, "CycleGraphs")

if __name__ == "__main__":
    main()
