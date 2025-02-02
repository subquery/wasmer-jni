package com.archeros.wasmer;

/**
 * TODO:
 * add options for blockchain application:
 * 1. gas metering & gas limit, limit memory page
 * 2. disallow start section
 * 3. force memory with name = memory export
 * 4. disallow global/memory/table import
 */
public class Options {
    private long threads;
    private long referenceTypes;
    private long simd;
    private long bulkMemory;
    private long multiValue;
    private long tailCall;
    private long moduleLinking;
    private long multiMemory;
    private long memory64;

    private Options() {
    }

    public static Options empty() {
        return new Options();
    }

    public Options threads(boolean threads) {
        this.threads = threads ? 1L: 0;
        return this;
    }

    public Options referenceTypes(boolean referenceTypes) {
        this.referenceTypes = referenceTypes ? (1L << 1): 0;
        return this;
    }

    public Options simd(boolean simd) {
        this.simd = simd ? (1L << 2): 0;
        return this;
    }

    public Options bulkMemory(boolean bulkMemory) {
        this.bulkMemory = bulkMemory ? (1L << 3): 0;
        return this;
    }

    public Options multiValue(boolean multiValue) {
        this.multiValue = multiValue ? (1L << 4): 0;
        return this;
    }

    public Options tailCall(boolean tailCall) {
        this.tailCall = tailCall ? (1L << 5): 0;
        return this;
    }

    public Options moduleLinking(boolean moduleLinking) {
        this.moduleLinking = moduleLinking ? (1L << 6): 0;
        return this;
    }

    public Options multiMemory(boolean multiMemory) {
        this.multiMemory = multiMemory ? (1L << 7): 0;
        return this;
    }

    public Options memory64(boolean memory64) {
        this.memory64 = memory64 ? (1L << 8): 0;
        return this;
    }

    long bitmap() {
        return threads | referenceTypes | simd | bulkMemory | multiValue | tailCall | moduleLinking | multiMemory | memory64;
    }
}
