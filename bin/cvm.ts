// @deno-types="https://cdn.skypack.dev/@types/lodash?dts"
import { shuffle } from "https://cdn.skypack.dev/lodash-es?dts";

class CVM<T> {
  size: number;
  seen: Set<T>;
  prob: number;

  constructor(size: number = 16384) {
    this.size = size;
    this.seen = new Set();
    this.prob = 1.0;
  }

  private halve(): void {
    const seen = Array.from(this.seen);
    shuffle(seen);
    this.seen = new Set(seen.slice(0, Math.floor(seen.length / 2)));
    this.prob /= 2;
  }

  public countItem(item: T) {
    if (Math.random() >= this.prob) {
      this.seen.delete(item);
    } else if (!this.seen.has(item)) {
      this.seen.add(item);
      if (this.seen.size >= this.size) this.halve();
    }
    return this;
  }

  public get cardinality(): number {
    return this.seen.size / this.prob;
  }
}

const cvm = new CVM<number>(16384);
for (let i = 0; i < 50_000_000; i++) cvm.countItem(i);
console.log(`deno: ${cvm.cardinality}`);

const cvm2 = new CVM<number>(16384);
for (let i = 0; i < 50_000_000; i++) cvm2.countItem(i % 1000);
console.log(`deno: ${cvm2.cardinality}`);
