import matplotlib.pyplot as plt
import csv

data = {}

with open("benchmark_res.csv", "r") as f:
    reader = csv.DictReader(f)
    for row in reader:
        n = int(row["n"])
        d = int(row["d"])
        time = float(row["time_ms"])

        if d not in data:
            data[d] = {"n": [], "time": []}

        data[d]["n"].append(n)
        data[d]["time"].append(time)

# plot
for d in sorted(data.keys()):
    plt.plot(data[d]["n"], data[d]["time"], label=f"d={d}")

plt.xlabel("Message size n")
plt.ylabel("Encoding time (ms)")
plt.title("Encoding Time vs n")
plt.legend()
plt.grid()
plt.show()