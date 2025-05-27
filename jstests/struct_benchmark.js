/**
 * Benchmark comparing DataView vs TypedArrays for a struct {a:u8, b:f32}
 * Tests read/write performance for different data sizes.
 */

// Define test sizes
const SMALL_SIZE = 1_000;
const MEDIUM_SIZE = 100_000;
const LARGE_SIZE = 1_000_000;
const ITERATIONS = 5; // Number of iterations for more accurate timing

// Size of our struct in bytes: u8 (1 byte) + f32 (4 bytes)
const STRUCT_SIZE = 5;

// Class for TypedArray implementation
class StructTypedArray {
  constructor(count) {
    // Allocate individual arrays for each field
    this.a = new Uint8Array(count);
    this.b = new Float32Array(count);
  }

  set(index, a, b) {
    this.a[index] = a;
    this.b[index] = b;
  }

  get(index) {
    return {
      a: this.a[index],
      b: this.b[index]
    };
  }
}

// Class for DataView implementation
class StructDataView {
  constructor(count) {
    // Allocate a single buffer for all elements
    this.buffer = new ArrayBuffer(count * STRUCT_SIZE);
    this.view = new DataView(this.buffer);
  }

  set(index, a, b) {
    const offset = index * STRUCT_SIZE;
    this.view.setUint8(offset, a);
    this.view.setFloat32(offset + 1, b, true); // true = little endian
  }

  get(index) {
    const offset = index * STRUCT_SIZE;
    return {
      a: this.view.getUint8(offset),
      b: this.view.getFloat32(offset + 1, true) // true = little endian
    };
  }
}

/**
 * Benchmarks write operations
 */
function benchmarkWrite(impl, count, name) {
  const start = performance.now();
  
  for (let i = 0; i < count; i++) {
    impl.set(i, i % 255, i * 1.1); // Simple values for testing
  }
  
  const end = performance.now();
  return end - start;
}

/**
 * Benchmarks read operations
 */
function benchmarkRead(impl, count, name) {
  // First write data so we have something to read
  for (let i = 0; i < count; i++) {
    impl.set(i, i % 255, i * 1.1);
  }
  
  // Now benchmark reading
  const start = performance.now();
  
  let a = 0;
  let b = 0;
  for (let i = 0; i < count; i++) {
    const val = impl.get(i);
    a += val.a;  // Use the values to prevent optimization
    b += val.b;
  }
  
  const end = performance.now();
  
  // Prevent optimizations from eliminating the loop
  if (a === 0 && b === 0) {
    console.log("This will never be printed but prevents optimization");
  }
  
  return end - start;
}

/**
 * Run all benchmarks for a specific size
 */
function runBenchmarks(size, sizeLabel) {
  console.log(`\n--- Running benchmarks for ${sizeLabel} size (${size.toLocaleString()} elements) ---`);
  
  const typedArrayTimes = { write: [], read: [] };
  const dataViewTimes = { write: [], read: [] };
  
  for (let i = 0; i < ITERATIONS; i++) {
    // Create fresh instances for each iteration
    const typedArray = new StructTypedArray(size);
    const dataView = new StructDataView(size);
    
    // Benchmark writes
    typedArrayTimes.write.push(benchmarkWrite(typedArray, size, "TypedArray"));
    dataViewTimes.write.push(benchmarkWrite(dataView, size, "DataView"));
    
    // Benchmark reads
    typedArrayTimes.read.push(benchmarkRead(typedArray, size, "TypedArray"));
    dataViewTimes.read.push(benchmarkRead(dataView, size, "DataView"));
  }
  
  // Calculate averages
  const avgTypedArrayWrite = typedArrayTimes.write.reduce((a, b) => a + b, 0) / ITERATIONS;
  const avgTypedArrayRead = typedArrayTimes.read.reduce((a, b) => a + b, 0) / ITERATIONS;
  const avgDataViewWrite = dataViewTimes.write.reduce((a, b) => a + b, 0) / ITERATIONS;
  const avgDataViewRead = dataViewTimes.read.reduce((a, b) => a + b, 0) / ITERATIONS;
  
  // Display results
  console.log(`\nResults (average of ${ITERATIONS} iterations):`);
  console.log("┌─────────────┬────────────────┬───────────────┬────────────────┐");
  console.log("│ Operation   │ TypedArray (ms)│ DataView (ms) │ TypedArray/DV  │");
  console.log("├─────────────┼────────────────┼───────────────┼────────────────┤");
  console.log(`│ Write       │ ${avgTypedArrayWrite.toFixed(2).padStart(14)} │ ${avgDataViewWrite.toFixed(2).padStart(13)} │ ${(avgTypedArrayWrite / avgDataViewWrite).toFixed(2).padStart(14)} │`);
  console.log(`│ Read        │ ${avgTypedArrayRead.toFixed(2).padStart(14)} │ ${avgDataViewRead.toFixed(2).padStart(13)} │ ${(avgTypedArrayRead / avgDataViewRead).toFixed(2).padStart(14)} │`);
  console.log("└─────────────┴────────────────┴───────────────┴────────────────┘");
  
  return {
    size: sizeLabel,
    count: size,
    typedArray: {
      write: avgTypedArrayWrite,
      read: avgTypedArrayRead
    },
    dataView: {
      write: avgDataViewWrite,
      read: avgDataViewRead
    },
    ratio: {
      write: avgTypedArrayWrite / avgDataViewWrite,
      read: avgTypedArrayRead / avgDataViewRead
    }
  };
}

// Run all benchmark sizes
console.log("BENCHMARK: TypedArrays vs DataView for struct {a:u8, b:f32}");
console.log(`Running ${ITERATIONS} iterations for each test`);

const results = [
  runBenchmarks(SMALL_SIZE, "Small"),
  runBenchmarks(MEDIUM_SIZE, "Medium"),
  runBenchmarks(LARGE_SIZE, "Large")
];

// Summary across all sizes
console.log("\n--- SUMMARY ---");
console.log("┌──────────┬───────────┬────────────┬────────────┬────────────┬────────────┬────────────┐");
console.log("│ Size     │ Elements  │ TA Write   │ DV Write   │ TA/DV Write│ TA Read    │ DV Read    │");
console.log("├──────────┼───────────┼────────────┼────────────┼────────────┼────────────┼────────────┤");

results.forEach(r => {
  console.log(`│ ${r.size.padEnd(8)} │ ${r.count.toString().padStart(9)} │ ${r.typedArray.write.toFixed(2).padStart(8)} ms │ ${r.dataView.write.toFixed(2).padStart(8)} ms │ ${r.ratio.write.toFixed(2).padStart(8)}x │ ${r.typedArray.read.toFixed(2).padStart(8)} ms │ ${r.dataView.read.toFixed(2).padStart(8)} ms │`);
});

console.log("└──────────┴───────────┴────────────┴────────────┴────────────┴────────────┴────────────┘");

// Provide analysis and recommendations
console.log("\n--- ANALYSIS ---");
console.log("• TypedArray ratio < 1.0 means TypedArray is faster than DataView");
console.log("• Higher element counts show the performance difference more clearly");
console.log("• TypedArrays generally perform better for both reading and writing operations");
console.log("• The performance gap between TypedArrays and DataView tends to increase with data size");
console.log("\n--- RECOMMENDATION ---");

// Check overall performance to make a recommendation
const avgWriteRatio = results.reduce((sum, r) => sum + r.ratio.write, 0) / results.length;
const avgReadRatio = results.reduce((sum, r) => sum + r.ratio.read, 0) / results.length;

if (avgWriteRatio < 1 && avgReadRatio < 1) {
  console.log("✓ Use TypedArrays for best overall performance");
} else if (avgWriteRatio < 1) {
  console.log("✓ Use TypedArrays for write-heavy operations");
} else if (avgReadRatio < 1) {
  console.log("✓ Use TypedArrays for read-heavy operations");
} else {
  console.log("✓ Results are mixed - consider your specific use case requirements");
}

// Additional memory consideration
console.log("\nNOTE: This benchmark focuses on performance, but memory usage is also important:");
console.log("• DataView uses a single contiguous buffer (more memory efficient)");
console.log("• TypedArrays implementation uses separate arrays (potentially more memory overhead)");
console.log("• For very large datasets, consider memory constraints alongside performance");

