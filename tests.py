import json
import subprocess
import time
import itertools
import csv

rust_executable = "target/release/nauty-colourings"  # Adjust if using release mode

def generate_cycle_graph(n):
    edges = [(i, (i + 1) % n) for i in range(n)]
    return {"name": f"Cycle_{n}", "vertices": n, "edges": edges}

def generate_star_graph(n):
    edges = [(0, i) for i in range(1, n)]
    return {"name": f"Star_{n}", "vertices": n, "edges": edges}

def generate_kneser_graph(n, k):
    vertices = list(itertools.combinations(range(n), k))
    edges = [(i, j) for i, a in enumerate(vertices) for j, b in enumerate(vertices) if i < j and set(a).isdisjoint(set(b))]
    return {"name": f"Kneser_{n}_{k}", "vertices": len(vertices), "edges": edges}

def generate_johnson_graph(n, k):
    vertices = list(itertools.combinations(range(n), k))
    edges = [(i, j) for i, a in enumerate(vertices) for j, b in enumerate(vertices) if i < j and len(set(a) ^ set(b)) == 2]
    return {"name": f"Johnson_{n}_{k}", "vertices": len(vertices), "edges": edges}

def run_test(graph):
    json_file = f"temp_{graph['name']}.json"
    with open(json_file, "w") as f:
        json.dump(graph, f)

    start_time = time.time()
    result = subprocess.run([rust_executable, json_file], capture_output=True, text=True)
    end_time = time.time()

    if result.returncode != 0:
        print(f"❌ ERROR: {graph['name']} failed!")
        print("STDOUT:", result.stdout)
        print("STDERR:", result.stderr)
        return None

    output = json.loads(result.stdout)
    output["runtime_ms"] = (end_time - start_time) * 1000
    return output

# Graph sizes to test
sizes_star = [10, 20, 50]
sizes_cycle = [5, 10, 15,20,30]

# Test cases
graphs = []

# Cycle and Star graphs
for n in sizes_star:
    graphs.append(generate_star_graph(n))
for n in sizes_cycle:
    graphs.append(generate_cycle_graph(n))
# Kneser and Johnson graphs
for n, k in [(5, 2),(6,2)]:  # Small values to avoid combinatorial explosion
    graphs.append(generate_kneser_graph(n, k))
    graphs.append(generate_johnson_graph(n, k))

# Run tests and save results
results = []
for graph in graphs:
    print(f"Testing {graph['name']}...")
    result = run_test(graph)
    if result:
        results.append(result)

# Write results to CSV
with open("graph_results.csv", "w") as f:
    writer = csv.DictWriter(f, fieldnames=["name", "vertices", "orbits", "runtime_ms"])
    writer.writeheader()
    writer.writerows(results)

print("✅ Testing complete. Results saved to graph_results.csv")

