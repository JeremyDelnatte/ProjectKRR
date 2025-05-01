#!/usr/bin/env python3
"""
benchmark_clingo.py – progressive benchmarking for a Clingo ASP program (Python API edition)
==========================================================================================
This version benchmarks a Clingo ASP model via the **Clingo Python API** and
plots *problem size* (`H×W×D`) against *mean runtime*.  The script grows the
parameter triple by always bumping the smallest dimension while keeping the
ordering `height ≤ width ≤ depth`.
"""
from __future__ import annotations

import argparse
import csv
import time
from pathlib import Path
from statistics import mean
from typing import Generator, Tuple

import clingo
import matplotlib.pyplot as plt
import pandas as pd
import secrets
import generator2

###############################################################################
# Helpers                                                                    #
###############################################################################

def progressive_triples(max_dim: int, steps: int | None = None, start_dim=2) -> Generator[Tuple[int, int, int], None, None]:
    """Yield triples (H, W, D) with non‑decreasing dimensions (H ≤ W ≤ D)."""
    h = w = d = start_dim
    count = 0
    while h <= max_dim and w <= max_dim and d <= max_dim:
        yield h, w, d
        count += 1
        if steps is not None and count >= steps:
            break

        if h == w == d:
            d += 1
        elif w == d and h < w:
            h += 1
        elif h == w and w < d:
            w += 1
        else:  # should never occur
            raise AssertionError("Broken ordering in progressive_triples")

###############################################################################
# Clingo runner (Python API)                                                 #
###############################################################################

def run_clingo(program: Path, h: int, w: int, d: int) -> float:
    """Solve once with Clingo and return wall‑clock seconds."""

    seed = secrets.randbelow(2**31)
    start = time.perf_counter()
    ctl = clingo.Control(["-c", f"height={h}", "-c", f"width={w}", "-c", f"depth={d}", "--rand-freq=1", "-t 11", f"--seed={str(seed)}"])
    ctl.load(str(program))
    ctl.ground([("base", [])])
    ctl.solve()
    return time.perf_counter() - start

def run_gen2(h: int, w: int, d: int) -> float:
    start = time.perf_counter()
    generator2.run_generator(h, w, d)
    return time.perf_counter() - start

def run_solver() -> float:
    start = time.perf_counter()
    generator2.run_solver()
    return time.perf_counter() - start

###############################################################################
# CSV helpers                                                                #
###############################################################################

def init_csv(path: Path):
    if not path.exists():
        with path.open("w", newline="") as fh:
            csv.writer(fh).writerow(["height", "width", "depth", "run", "seconds", "size"])


def append_csv(path: Path, h: int, w: int, d: int, run_idx: int, seconds: float):
    with path.open("a", newline="") as fh:
        csv.writer(fh).writerow([h, w, d, run_idx, f"{seconds:.6f}", h * w * d])


def aggregate(csv_path: Path) -> pd.DataFrame:
    df = pd.read_csv(csv_path)
    grp = df.groupby(["height", "width", "depth", "size"]).agg(mean_seconds=("seconds", mean)).reset_index()
    grp.sort_values("size", inplace=True)
    return grp

###############################################################################
# Plotting                                                                   #
###############################################################################

def plot_size_vs_time(df: pd.DataFrame, out_svg: Path | None):
    fig, ax = plt.subplots()
    ax.scatter(df["size"], df["mean_seconds"])

    for _, row in df.iterrows():
        x = float(row["size"])
        y = float(row["mean_seconds"])
        ax.annotate(
            f"{int(row['size'])}",
            xy=(x, y),
            textcoords="offset points",
            xytext=(0, 5),
            ha="center",
        )

    ax.set_xlabel("Problem size (H × W × D)")
    ax.set_ylabel("Mean runtime (s, log‑scale)")
    ax.set_yscale("log")
    ax.set_title("Clingo runtime vs. problem size")
    ax.grid(True, which="both", linestyle=":", linewidth=0.5)
    fig.tight_layout()
    if out_svg:
        fig.savefig(out_svg, format="svg")
        print(f"Saved plot to {out_svg}")
    else:
        plt.show()

###############################################################################
# CLI                                                                        #
###############################################################################

def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Benchmark a Clingo ASP program with progressive parameter growth (Python API).")
    p.add_argument("--program", required=False, type=Path, help="Path to the .lp file.")
    p.add_argument("--max-dim", required=False, type=int, help="Stop when any dimension exceeds this value.")
    p.add_argument("--start-dim", required=False, default=2, type=int)
    p.add_argument("--steps", type=int, help="Optional cap on the number of triples to benchmark.")
    p.add_argument("--repeats", type=int, default=5, help="Repetitions per triple (default: 5).")
    p.add_argument("--outcsv", type=Path, default=Path("bench_results.csv"), help="CSV file for incremental logging.")
    p.add_argument("--outsheet", type=Path, help="Save the final SVG plot here.")
    p.add_argument("--genpy", action="store_true", help="Use the Python generator")
    p.add_argument("--solve", action="store_true", help="Solves the instance")
    return p.parse_args()

###############################################################################
# Main                                                                       #
###############################################################################

def main():
    solver_outcsv = Path("bench_results_solver.csv")
    solver_outsvg = Path("bench_results_solver.svg")
    args = parse_args()
    init_csv(args.outcsv)
    init_csv(solver_outcsv)

    if args.program is not None or args.genpy:
        triples = list(progressive_triples(args.max_dim, args.steps, args.start_dim))
        total_runs = len(triples) * args.repeats

        current = 0
        for h, w, d in triples:
            for run_idx in range(1, args.repeats + 1):
                current += 1
                print(f"[{current}/{total_runs}] h={h} w={w} d={d} run={run_idx} …", end=" ")

                if args.genpy:
                    secs = run_gen2(h, w, d)

                else:
                    secs = run_clingo(args.program, h, w, d)

                append_csv(args.outcsv, h, w, d, run_idx, secs)
                print(f"{secs:.3f}s", end=" ")

                if args.solve:
                    secs = run_solver()
                    append_csv(solver_outcsv, h, w, d, run_idx, secs)
                    print(f"solve={secs:.3f}s", end=" ")

                print()


    df = aggregate(args.outcsv)
    plot_size_vs_time(df, args.outsheet)

    df = aggregate(solver_outcsv)
    plot_size_vs_time(df, solver_outsvg)


if __name__ == "__main__":
    main()
