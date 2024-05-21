import os
import json
import pandas as pd
import matplotlib.pyplot as plt

def parse_benchmark(path):
    with open(path) as f:
        data = json.load(f)
    times = data['mean']['point_estimate']
    return times

def main():
    base_dir = "target/criterion"
    benchmarks = ["threads_1", "threads_2", "threads_3", "threads_4"]
    results = {}

    for bench in benchmarks:
        bench_path = os.path.join(base_dir, bench, "new", "estimates.json")
        results[bench] = parse_benchmark(bench_path)

    df = pd.DataFrame.from_dict(results, orient='index', columns=['time_ns'])
    df.index = df.index.str.replace('threads_', '').astype(int)
    df.sort_index(inplace=True)

    plt.figure(figsize=(10, 6))
    plt.plot(df.index, df['time_ns'] / 1e6, marker='o')
    plt.title('Performance of Finding Primes with Different Thread Counts')
    plt.xlabel('Number of Threads')
    plt.ylabel('Time (ms)')
    plt.grid(True)
    plt.savefig('output/benchmark_results.png')  # Сохранение графика в файл

if __name__ == "__main__":
    main()
