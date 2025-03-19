import json
import subprocess
import csv
import time

def generate_graph(name, n, graph_type):
    """Generates a graph description."""
    if graph_type == "cycle":
        edges = [(i, (i+1) % n) for i in range(n)]
    elif graph_type == "star":
        edges = [(0, i) for i in range(1, n)]
    else:
        raise ValueError("Unknown graph type")
    
    return {"name": name, "vertices": n, "edges": edges}

def run_test(graph, rust_executable):
    """Runs the Rust program on the given graph."""
    json_file = f"temp_{graph['name']}.json"
    with open(json_file, "w") as f:
        json.dump(graph, f)
    
    start_time = time.time()
    result = subprocess.run([rust_executable, json_file], capture_output=True, text=True)
    duration = (time.time() - start_time) * 1000
    
    if result.returncode != 0:
        print(f"Error running Rust program: {result.stderr}")
        return None
    
    output = json.loads(result.stdout)
    output["runtime_ms"] = duration  # Adding actual measured runtime
    return output

def main():
    rust_executable = "./target/release/nauty-colourings"
    test_cases = [("Cycle", n, "cycle") for n in [4,5,7,10,12]] + \
                 [("Star", n, "star") for n in [10, 20,25,32]]
    
    # test_cases = [("Star", n, "star") for n in [10, 20,25,32]]
    results = []
    for name, n, g_type in test_cases:
        graph = generate_graph(f"{name}_{n}", n, g_type)
        print(f"Testing {name} graph with {n} vertices...")
        result = run_test(graph, rust_executable)
        if result:
            results.append(result)
    
    with open("results.csv", "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["name", "vertices", "orbits", "runtime_ms"])
        writer.writeheader()
        writer.writerows(results)
    
    print("Testing complete. Results saved to results.csv.")

if __name__ == "__main__":
    main()

