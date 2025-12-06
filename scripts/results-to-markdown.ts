import { readdir } from "node:fs/promises";
import { join } from "node:path";

const CRITERION_DIR = "./target/criterion";

interface Estimate {
  point_estimate: number;
  standard_error: number;
  confidence_interval: {
    lower_bound: number;
    upper_bound: number;
    confidence_level: number;
  };
}

interface Estimates {
  mean: Estimate;
  median: Estimate;
  median_abs_dev: Estimate;
  slope?: Estimate;
  std_dev: Estimate;
}

interface BenchmarkResult {
  name: string;
  mean: number;
  stdDev: number;
  lowerBound: number;
  upperBound: number;
}

function formatTime(ns: number): string {
  if (ns < 1000) {
    return `${ns.toFixed(2)} ns`;
  } else if (ns < 1_000_000) {
    return `${(ns / 1000).toFixed(2)} µs`;
  } else if (ns < 1_000_000_000) {
    return `${(ns / 1_000_000).toFixed(2)} ms`;
  } else {
    return `${(ns / 1_000_000_000).toFixed(2)} s`;
  }
}

async function readBenchmarkResult(
  benchmarkDir: string,
): Promise<BenchmarkResult | null> {
  const estimatesPath = join(benchmarkDir, "new", "estimates.json");

  try {
    const file = Bun.file(estimatesPath);
    const exists = await file.exists();
    if (!exists) {
      return null;
    }

    const estimates: Estimates = await file.json();
    const name = benchmarkDir.split("/").pop() || "unknown";

    return {
      name,
      mean: estimates.mean.point_estimate,
      stdDev: estimates.std_dev.point_estimate,
      lowerBound: estimates.mean.confidence_interval.lower_bound,
      upperBound: estimates.mean.confidence_interval.upper_bound,
    };
  } catch {
    return null;
  }
}

async function main() {
  const entries = await readdir(CRITERION_DIR, { withFileTypes: true });

  const benchmarkDirs = entries
    .filter((entry) => entry.isDirectory() && entry.name !== "report")
    .map((entry) => join(CRITERION_DIR, entry.name));

  const results: BenchmarkResult[] = [];

  for (const dir of benchmarkDirs) {
    const result = await readBenchmarkResult(dir);
    if (result) {
      results.push(result);
    }
  }

  if (results.length === 0) {
    console.error("No benchmark results found!");
    console.error("Run `cargo bench` first to generate results.");
    process.exit(1);
  }

  // Sort by mean time (fastest first)
  results.sort((a, b) => a.mean - b.mean);

  // Separate string concat benchmarks from RGB benchmarks
  const stringResults = results.filter((r) => !r.name.startsWith("rgb_"));
  const rgbResults = results.filter((r) => r.name.startsWith("rgb_"));

  const generateTable = (
    benchmarks: BenchmarkResult[],
    title: string,
  ): string => {
    if (benchmarks.length === 0) return "";

    const fastest = benchmarks[0].mean;

    let table = `## ${title}\n\n`;
    table += "| Rank | Benchmark | Mean | Std Dev | vs Fastest |\n";
    table += "|------|-----------|------|---------|------------|\n";

    benchmarks.forEach((result, index) => {
      const ratio = result.mean / fastest;
      const ratioStr =
        ratio === 1 ? "1.00x (baseline)" : `${ratio.toFixed(2)}x slower`;

      table += `| ${index + 1} | \`${result.name}\` | ${formatTime(result.mean)} | ±${formatTime(result.stdDev)} | ${ratioStr} |\n`;
    });

    return table;
  };

  let output = "# Concatenation Benchmark Results\n\n";
  output += `Generated: ${new Date().toISOString()}\n\n`;

  if (stringResults.length > 0) {
    output += generateTable(stringResults, "String Concatenation (DateTime)");
    output += "\n";
  }

  if (rgbResults.length > 0) {
    output += generateTable(
      rgbResults,
      "RGB Formatting (with number conversion)",
    );
    output += "\n";
  }

  // Summary section
  output += "## Summary\n\n";

  if (stringResults.length > 0) {
    output += `**Fastest string concat:** \`${stringResults[0].name}\` (${formatTime(stringResults[0].mean)})\n\n`;
  }

  if (rgbResults.length > 0) {
    output += `**Fastest RGB format:** \`${rgbResults[0].name}\` (${formatTime(rgbResults[0].mean)})\n\n`;
  }

  console.log(output);

  // Also write to file
  const outputPath = "./BENCHMARK_RESULTS.md";
  await Bun.write(outputPath, output);
  console.error(`\nResults written to ${outputPath}`);
}

main().catch(console.error);
