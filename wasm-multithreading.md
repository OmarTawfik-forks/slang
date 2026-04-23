# Rust Multithreading in Browser WebAssembly — State of the Art (April 2026)

## TL;DR

Rust code can run multithreaded in the browser today, but the path is still narrower and sharper-edged than native multithreading. The substrate (WebAssembly threads + `SharedArrayBuffer` + `Atomics`) is shipping in every major desktop and mobile browser, gated by cross-origin isolation (COOP/COEP headers). On the Rust side, the `wasm32-unknown-unknown` target plus `-C target-feature=+atomics,+bulk-memory,+mutable-globals` and a nightly `-Z build-std` rebuild of `std` is still the canonical recipe. `wasm-bindgen-rayon` remains the de-facto library for data-parallel workloads, while `wasm_thread` offers a closer `std::thread` analog. Async (`tokio`, `wasm-bindgen-futures`) is single-threaded in the browser and generally complementary to, not a replacement for, real threads.

## Production-readiness summary table

| Item | Tag | Why |
| --- | --- | --- |
| Wasm threads proposal | [Production-ready] | Phase 4, shipping in all evergreen browsers |
| `SharedArrayBuffer` | [Mostly production-ready (caveats)] | Ships everywhere, requires COOP/COEP isolation |
| `Atomics.wait` / `notify` | [Mostly production-ready (caveats)] | Blocks only on dedicated Workers; not on main thread |
| `Atomics.waitAsync` | [Mostly production-ready (caveats)] | Firefox shipped in 145 (November 2025); now in all major browsers |
| Cross-origin isolation (COOP/COEP) | [Mostly production-ready (caveats)] | Works but breaks third-party embeds without CORP headers |
| `wasm32-unknown-unknown` (Rust) | [Production-ready] | Tier 2 without host tools; stable for single-threaded |
| Threaded wasm (atomics + shared memory) | [Mostly production-ready (caveats)] | Requires nightly `-Z build-std`; otherwise stable in practice |
| `wasm32-wasip1-threads` | [Mostly production-ready (caveats)] | Tier 2, but targets WASI runtimes, not browsers |
| `wasm-bindgen` | [Production-ready] | De-facto standard, widely deployed |
| `wasm-bindgen-rayon` | [Mostly production-ready (caveats)] | 1.3.0, post-1.0 and API stable; requires bundler setup and COOP/COEP |
| `wasm_thread` | [Mostly production-ready (caveats)] | Pre-1.0; closer `std::thread` feel, fewer users |
| `rayon` (upstream, in browser) | [Mostly production-ready (caveats)] | Used via `wasm-bindgen-rayon` shim |
| `tokio` (in browser) | [Mostly production-ready (caveats)] | Single-threaded `current_thread` runtime only |
| `parking_lot` | [Mostly production-ready (caveats)] | Works with `+atomics`, but `std::sync` is usually enough |
| Web Workers / `postMessage` | [Production-ready] | Universal, stable for 15+ years |
| `shared-everything-threads` (Component Model) | [Not production-ready] | Proposal phase, no shipping implementation |

## 1. The substrate: WebAssembly threads, SAB, cross-origin isolation

Multithreaded WebAssembly in the browser rests on three interlocking pieces: the Wasm threads proposal, `SharedArrayBuffer` + `Atomics` on the JS side, and cross-origin isolation at the HTTP level. All three must line up or nothing works.

### 1.1 The Wasm threads proposal [Production-ready]

The [WebAssembly threads proposal](https://github.com/WebAssembly/threads) reached **Phase 4** (standardization) in early 2024. It adds:

- A `shared` flag on linear memories, allowing a single `WebAssembly.Memory` to be imported by multiple module instances across Workers.
- Atomic memory instructions (`i32.atomic.load`, `i32.atomic.rmw.add`, `memory.atomic.wait32`, `memory.atomic.notify`, etc.) that map onto hardware atomics and the JS `Atomics` object.
- `memory.atomic.wait` / `memory.atomic.notify` for cross-thread blocking/wakeup, mirroring `Atomics.wait` / `Atomics.notify`.

Shipping status as of early 2026:

- **V8** (Chrome, Edge, Opera, Node.js): shipped since Chrome 74 (2019) behind origin-trial, unflagged since Chrome 91 (2021).
- **SpiderMonkey** (Firefox): shipped since Firefox 79 (2020).
- **JavaScriptCore** (Safari): `SharedArrayBuffer` available since **Safari 15.2** (December 2021); full wasm threads support (shared memory import) landed in later versions (16.x) on both macOS and iOS/iPadOS.

That makes threaded wasm available on every evergreen browser, though older iOS devices without a recent Safari update are shut out.

### 1.2 `SharedArrayBuffer` and `Atomics` [Mostly production-ready (caveats)]

`SharedArrayBuffer` ([MDN](https://developer.mozilla.org/en-US/docs/Web/API/SharedArrayBuffer)) is the JS-visible backing store for shared wasm memory, and the [`Atomics`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics) object exposes the same atomic primitives at the JS level (`Atomics.load`, `Atomics.store`, `Atomics.add`, `Atomics.compareExchange`, `Atomics.wait`, `Atomics.notify`, `Atomics.waitAsync`).

Every browser that ships Wasm threads also ships `SharedArrayBuffer` — they are effectively one feature, because a shared `WebAssembly.Memory` exposes its bytes to JS as a `SharedArrayBuffer`.

### 1.3 Cross-origin isolation [Mostly production-ready (caveats)]

After the Spectre/Meltdown disclosures, browsers disabled `SharedArrayBuffer` by default. To re-enable it, the top-level document must be **cross-origin isolated**, which requires two response headers on the main HTML document:

```http
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Alternatively, since 2022 Chromium (and later Firefox and Safari) supports `Cross-Origin-Embedder-Policy: credentialless`, which allows loading cross-origin no-credential subresources without explicit `Cross-Origin-Resource-Policy` headers on each one. See [web.dev: COOP/COEP](https://web.dev/articles/coop-coep).

Without these headers, `self.crossOriginIsolated` is `false`, `SharedArrayBuffer` is not constructible, and instantiating a wasm module with `shared` memory will fail. Every cross-origin subresource (images, scripts, fonts, iframes) then either needs `Cross-Origin-Resource-Policy: cross-origin`, a CORS handshake, or must be swapped for a same-origin copy.

### 1.4 Shared `WebAssembly.Memory` [Production-ready]

A shared memory is constructed as:

```js
const memory = new WebAssembly.Memory({
  initial: 256,     // pages (64 KiB each)
  maximum: 16384,   // required when shared: true
  shared: true,
});
```

The `maximum` field is mandatory for shared memories — the engine needs an upper bound so it can reserve address space rather than move the buffer on growth. Each Worker that wants to participate instantiates the same `WebAssembly.Module` with this `memory` passed in as an import.

## 2. Rust compiler targets

The Rust ecosystem exposes several wasm targets; for browser threading only one is really relevant, but the others are worth contrasting.

### 2.1 `wasm32-unknown-unknown` [Production-ready single-threaded; Mostly production-ready with caveats for threads]

This is the canonical browser wasm target. It is **Tier 2 without host tools** in the Rust [platform support matrix](https://doc.rust-lang.org/rustc/platform-support.html) (wasm targets can't host a Rust toolchain) and ships with every stable toolchain.

For a threaded build you must enable the relevant wasm features:

```sh
RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals" \
  cargo +nightly build --target wasm32-unknown-unknown \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort
```

A few things are glued together here:

- `+atomics` enables atomic memory instructions and is the feature gate for shared memory.
- `+bulk-memory` is required because atomic ops generate `memory.fill` / `memory.copy` in some paths, and it is also a prerequisite for threads.
- `+mutable-globals` is required so that TLS (`#[thread_local]`) can be implemented via per-instance mutable globals.
- `-Z build-std` rebuilds `libstd` with these features. The precompiled sysroot shipped with rustup is compiled **without** `+atomics`, so `std::sync::Mutex`, `std::thread::spawn`, and friends use relaxed codegen that does not interoperate with actual shared memory. You need nightly for this today.
- `panic_immediate_abort` is optional but keeps code size manageable since unwinding across wasm threads is a mess.

Once built, `std::thread::spawn` works, `std::sync::{Mutex, RwLock, Condvar}` works, `std::sync::mpsc` works, `Arc` / `atomic` types work — as long as something on the JS side (typically `wasm-bindgen-rayon`) creates the Workers that will host those threads. Raw Rust cannot spawn a Worker by itself; a thread handle maps to a Worker via JS glue.

### 2.2 `wasm32-wasip1-threads` [Mostly production-ready]

Formerly called `wasm32-wasi-preview1-threads`, this target is **Tier 2**. It adds the `wasi-threads` ABI so that `std::thread::spawn` can call out to the host runtime (typically [wasmtime](https://wasmtime.dev/)) to create new threads.

It is **not** a browser target. Browsers do not implement the `wasi-threads` imports. It is mentioned here only to disambiguate: if you want threads in the browser, this is the wrong target.

### 2.3 Component Model / `wasm32-wasip2` [Experimental for threading]

`wasm32-wasip2` is the Component-Model-facing target (WASI 0.2, then 0.3). The Component Model itself ships in wasmtime and jco, but:

- Browsers do not natively execute components — a polyfill like `jco transpile` is required.
- Threads in the Component Model depend on the [shared-everything threads](https://github.com/WebAssembly/shared-everything-threads) proposal, which as of early 2026 is still an early-stage proposal (draft status on GitHub) with no shipping browser implementation.

So: components + browser + threads is an aspirational combination today, not a production one.

### 2.4 Build tooling

You rarely invoke `cargo` this way directly. In practice:

- **`wasm-pack`** — handles the target, `wasm-bindgen` post-processing, and packaging for npm. Setting the build-std flags via `.cargo/config.toml` is still required for threaded builds.
- **Trunk** — end-to-end bundler for Rust-first web apps; supports threaded builds once you enable the `atomics` target feature.
- **`wasm-bindgen-rayon`'s build script** — expects you to set `RUSTFLAGS` and use nightly + `build-std` in your `.cargo/config.toml`. Its README has a copy-pasteable recipe.

## 3. Rust-side libraries

### 3.1 `wasm-bindgen` [Production-ready]

[`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/) is the glue layer between Rust and JS: it generates JS shims, marshals strings and structs, and exposes JS APIs to Rust via `#[wasm_bindgen]`. It is pre-1.0 (`0.2.x` series, currently 0.2.118 as of April 2026) but has been API-stable for years and is used by essentially every Rust-in-browser project. Thread-related helpers (`wasm-bindgen-futures`, `wasm_bindgen::memory()` for obtaining the shared memory handle) live in the same family of crates.

### 3.2 `wasm-bindgen-rayon` [Mostly production-ready (caveats)]

[`wasm-bindgen-rayon`](https://github.com/RReverser/wasm-bindgen-rayon) is the drop-in that makes `rayon` actually work in the browser. Originally developed under GoogleChromeLabs; that repo was archived in July 2024 and development continues on RReverser's fork. It ships:

- A JS bootstrap (`init_thread_pool`) that spawns N Web Workers, each of which loads the same `WebAssembly.Module` and instantiates it against the shared `WebAssembly.Memory`.
- TLS and panic-hook setup so that each Worker can safely host a Rust thread.
- A patched rayon `ThreadBuilder` that, instead of calling `std::thread::spawn`, hands each thread's entry point to a pre-spawned Worker.

Caveats:

- Current version is **1.3.0** (post-1.0, API stable).
- Requires nightly Rust for `-Z build-std`.
- Requires the hosting page to be cross-origin isolated.
- The bundler story is fiddly: Webpack, Rollup, and Vite each need a bit of configuration to emit the Worker entry point correctly, and to serve it with the right MIME type.

Despite all that, it is the most battle-tested option — Google's Squoosh image compressor, among others, has shipped it in production since 2020.

### 3.3 `rayon` upstream [Production-ready on native; Mostly production-ready in browser via the shim]

[`rayon`](https://crates.io/crates/rayon) itself needs no patches. Once `wasm-bindgen-rayon::init_thread_pool(n).await` has run, all the usual `par_iter`, `join`, `scope`, and `ThreadPoolBuilder` APIs behave exactly as on native.

### 3.4 `wasm_thread` [Mostly production-ready (caveats)]

[`wasm_thread`](https://crates.io/crates/wasm_thread) is a thin `std::thread::spawn`-compatible wrapper backed by Web Workers. Unlike rayon's fixed pool, it creates a Worker per spawned thread, so it is a better fit when you want a small number of long-lived, differently-shaped threads (a compiler worker, a networking worker, a UI worker) rather than a data-parallel pool. It is pre-1.0 (currently 0.3.3) and has a smaller user base than `wasm-bindgen-rayon`, but the code is small and easy to audit.

### 3.5 `tokio` in the browser [Mostly production-ready (caveats)]

`tokio` compiles to `wasm32-unknown-unknown`, but only with the `rt` (not `rt-multi-thread`) feature, i.e. only the `current_thread` runtime. The multi-thread scheduler calls `std::thread::spawn` from within the runtime's internals in ways that assume a native OS, and those paths do not play nicely with Worker-based spawning even under `+atomics`.

In practice: run one `tokio::runtime::Builder::new_current_thread()` on the main thread (or inside a dedicated Worker), and use `wasm-bindgen-rayon` or `wasm_thread` for CPU parallelism separately. Don't try to force `rt-multi-thread` on.

### 3.6 `wasm-bindgen-futures` [Production-ready]

[`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures) bridges Rust `Future`s to JS Promises and microtasks. It is what lets you `await` a `fetch()` call from Rust. It is single-threaded — it runs futures on the current JS event loop — and is unrelated to multithreading, but nearly every async-plus-threads project uses both.

### 3.7 `std::sync` primitives [Production-ready for the primitives themselves]

Under `-C target-feature=+atomics` with a rebuilt std, `Mutex`, `RwLock`, `Condvar`, `Barrier`, `Once`, `mpsc` channels, and `Arc` all lower to real atomic ops and `memory.atomic.wait` / `memory.atomic.notify`. The primitives themselves work; what can go wrong is calling the blocking ones from the main thread (see Gotchas).

### 3.8 `parking_lot` and `crossbeam` [Mostly production-ready (caveats)]

[`parking_lot`](https://crates.io/crates/parking_lot) and [`crossbeam`](https://crates.io/crates/crossbeam) both compile and work under `+atomics` with no wasm-specific changes. `crossbeam-channel`, `crossbeam-deque`, and `crossbeam-utils` are fine; `crossbeam-epoch` works but its TLS usage means each Worker pays a small per-thread bookkeeping cost. The only real caveat is the same as for `std::sync`: do not hold a blocking lock on the main thread.

## 4. JavaScript-side APIs

On the JS side, browser threading for wasm is assembled out of Web Workers, `postMessage`, shared memory, and atomics. None of these APIs is wasm-specific — they predate Wasm threads by years — but a specific combination of them is the canonical "wasm thread pool" pattern.

### 4.1 Dedicated Web Workers [Production-ready]

A [dedicated Web Worker](https://developer.mozilla.org/en-US/docs/Web/API/Worker) is the unit of parallelism. Each Rust "thread" in a threaded wasm program corresponds to one Worker that has `importScripts`'d (classic) or `import`'d (module) the generated `wasm-bindgen` JS glue and then called `WebAssembly.instantiate(module, imports)` with the **same** `WebAssembly.Module` and the **same** shared `WebAssembly.Memory` that the main thread is using. All Workers therefore see the same linear memory bytes at the same indices.

Module Workers (`new Worker(url, { type: 'module' })`) are now supported across all major browsers and are the preferred mode, as they allow static `import` statements inside the worker script and integrate cleanly with bundlers.

### 4.2 Shared Workers and Service Workers [Production-ready in their roles, not the right tool here]

[Shared Workers](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker) are shared across same-origin browsing contexts and could in principle host a wasm instance. In practice they are not used for Rust thread pools: their module-worker support has been inconsistent across browsers (Safari in particular was late), and their lifetime model does not match a compute pool. [Service Workers](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API) are network-interception proxies, not compute workers — they can run wasm but should not be used for CPU parallelism.

### 4.3 `postMessage` and transferables [Production-ready]

The handshake between the main thread and each pool Worker goes over `postMessage`. You typically post a small bootstrap payload containing the `WebAssembly.Module` handle and the shared `WebAssembly.Memory` handle (both are [structured-cloneable](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm)), plus any initial parameters. Large owned `ArrayBuffer`s can be moved zero-copy via the [transfer list](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage) (second argument to `postMessage`), which detaches the buffer in the sender.

### 4.4 `SharedArrayBuffer` [Mostly production-ready (caveats)]

Constructing a `SharedArrayBuffer` directly requires `self.crossOriginIsolated === true`. In a wasm context you rarely construct one directly: a `WebAssembly.Memory` with `shared: true` exposes its `.buffer` as a `SharedArrayBuffer` automatically, and that is what gets passed around.

### 4.5 `Atomics.wait` and `Atomics.notify` [Mostly production-ready (caveats)]

[`Atomics.wait(view, index, value, timeout)`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics/wait) blocks the calling agent until the slot changes or the timeout elapses. Critically, it **throws** when called on the main thread (its spec predicate is `[[CanBlock]]`, which is false for the window agent). This is the root cause of most "my tab froze" incidents: Rust's `Mutex::lock()` ultimately lowers to `memory.atomic.wait`, which lowers to `Atomics.wait` semantics, which is forbidden on the main thread. `Atomics.notify` has no such restriction.

### 4.6 `Atomics.waitAsync` [Mostly production-ready (caveats)]

[`Atomics.waitAsync`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics/waitAsync) is the non-blocking variant — it returns `{ async: true, value: Promise<"ok" | "timed-out"> }` and is safe to call on the main thread. It shipped first in Chromium (circa 2021), then Safari, and Firefox shipped it in **Firefox 145 (November 2025)**. As of early 2026 it is available in every evergreen browser. It is what a well-written lock-free wakeup scheme should use when the caller might be the main thread.

### 4.7 Shared `WebAssembly.Module` and `WebAssembly.Memory` [Production-ready]

`WebAssembly.Module` is postMessage-transferable (structured cloneable) since the early threads work, so the main thread can compile once and hand the compiled module out to each Worker. Each Worker then calls `WebAssembly.instantiate(mod, { env: { memory, ... } })` with the shared memory as an import. This avoids N compilations of the same bytes.

### 4.8 Worker-pool bootstrap pattern

The pattern that `wasm-bindgen-rayon` implements, and that every hand-rolled pool ends up converging on, is:

1. Main thread: compile the module, build a shared `WebAssembly.Memory`, instantiate once for itself.
2. Main thread: spawn `N` dedicated Workers (typically `navigator.hardwareConcurrency`), `postMessage` the module + memory to each.
3. Each Worker: instantiate the module with the shared memory, run wasm-bindgen's TLS/panic-hook setup, then post a ready message back.
4. Main thread: once all Workers are ready, call the Rust-side entry point that starts rayon's `ThreadPool` whose "spawn" operation hands a closure pointer to a waiting Worker over postMessage or a shared SPSC queue.

### 4.9 Comlink [Production-ready]

[Comlink](https://github.com/GoogleChromeLabs/comlink) wraps `postMessage` in a transparent Promise-based RPC. It is complementary to, not a replacement for, threading — useful for offloading a discrete task to a Worker, not for building a shared-memory compute pool. Many apps use both: Comlink to talk to a "coordinator" worker, and wasm-bindgen-rayon inside that worker for data parallelism.

### 4.10 Bundler integration

Modern bundlers can emit worker entry points from the `new Worker(new URL('./worker.js', import.meta.url), { type: 'module' })` pattern:

- **Vite** handles this natively; COOP/COEP for the dev server typically comes from a plugin like [`vite-plugin-cross-origin-isolation`](https://github.com/chaosprint/vite-plugin-cross-origin-isolation).
- **webpack 5** recognizes the same pattern via asset modules / `new Worker(new URL(...))`.
- **Rollup** needs `@rollup/plugin-url` or a dedicated worker plugin.
- **Trunk** and **wasm-pack** handle the Rust side end-to-end; the web server still has to send COOP/COEP.

COOP/COEP must be set by the actual HTTP server (CDN, nginx, Cloudflare Worker, static host config), not by the bundler at build time, since they are response headers on the top-level HTML document.

## 5. Performance characteristics

The performance story is "close to native, if you amortize the setup and don't cross the JS/wasm boundary too often". Numbers below are qualitative orders of magnitude; your mileage will vary by browser, CPU, and workload.

### 5.1 Worker boot cost

Creating a dedicated Worker, instantiating the compiled module in it, and running wasm-bindgen's per-Worker TLS/init takes on the order of **tens to low hundreds of milliseconds** cold. The compile step is the dominant cost the first time; subsequent Workers reuse the already-compiled `WebAssembly.Module`. The practical implication is that you pool Workers at app startup (or just after) rather than spin one up per task.

### 5.2 Atomics and contention

Uncontended atomic loads/stores/RMWs on shared memory are lowered to the same hardware instructions as native atomics and are essentially free relative to the surrounding work. Contended `Atomics.wait` / `memory.atomic.wait` has extra cost because the engine has to manage the agent's suspend/resume outside wasm — typically within 2-3x of a native futex wait, though the actual ratio varies widely by engine. For rayon-style fork/join workloads this is usually invisible.

### 5.3 `postMessage` cost

`postMessage` with a transferable (`ArrayBuffer` moved via the transfer list) is effectively zero-copy and takes constant time regardless of buffer size. `postMessage` of a structured-cloned object scales with its size and can dominate on large payloads — which is why every mature pool design shares state through the shared memory, not through message passing.

### 5.4 Per-worker memory

Each Worker has its own JS heap, its own compiled-code cache, and its own wasm-bindgen JS-glue state. Empirically this is several MB of RSS per Worker before any user allocation, on top of the single shared wasm linear memory. Pools of 4-8 Workers are unremarkable; pools of 64+ start to push memory on low-end devices.

### 5.5 Real-world deployments

- **Squoosh** (Google Chrome Labs) was the poster-child adopter of `wasm-bindgen-rayon`, running image encoders (mozjpeg, OxiPNG, AVIF) in parallel Workers.
- **Figma** ships large amounts of WASM for its document engine; its threading story has not been publicly detailed.
- **Google Meet**, **Zoom**, and **Microsoft Teams** use WASM-based audio/video codecs in the browser; several of these rely on wasm threads for real-time work.
- **Photoshop on the Web** (Adobe) is a canonical large-scale threaded-WASM deployment.

### 5.6 When threading isn't worth it

- Tasks whose per-invocation cost is **under ~1 ms**: the postMessage/wakeup round-trip is comparable to or larger than the work itself.
- Workloads dominated by JS<->wasm boundary crossings (lots of small `#[wasm_bindgen]` calls): parallelism doesn't help when the bottleneck is marshalling.
- Mobile Safari / older iPads: memory pressure and process limits make large Worker pools risky.
- Single-threaded workloads that are already I/O-bound: use `wasm-bindgen-futures` with async, not threads.

## 6. Gotchas

### 6.1 Mobile Safari and older iOS

`SharedArrayBuffer` has been available since **Safari 15.2** (December 2021) on both macOS and iOS/iPadOS, with full wasm threads support (shared memory import) landing in later 16.x versions. Older devices that cannot update past iOS 14 — which includes some iPhones/iPads still in active use — do not support shared memory at all. Even on supported iOS, Worker memory limits are tighter than on desktop, and tabs can be background-killed more aggressively. Design a single-threaded fallback path.

### 6.2 COEP and third-party content

Turning on COEP `require-corp` is the single most disruptive operational change. Every cross-origin subresource (images, fonts, ad iframes, analytics pixels) must either:

- Send `Cross-Origin-Resource-Policy: cross-origin` (or `same-site`), or
- Be loaded with proper CORS (`crossorigin` attribute + server CORS headers), or
- Be served same-origin.

In practice this breaks many ad networks, third-party embeds (YouTube, Twitter/X, Disqus), and analytics vendors until they update their headers. `COEP: credentialless` softens this by allowing cross-origin no-credential loads without CORP, and is the right default for most sites starting a migration today.

### 6.3 `Atomics.waitAsync` browser compatibility

For a while, `Atomics.waitAsync` was Chromium-only, and articles from 2021-2023 reflect that. Safari followed. **Firefox shipped `Atomics.waitAsync` in Firefox 145 (November 2025)**, so as of early 2026 it is available everywhere. Older documentation claiming otherwise is stale.

### 6.4 TLS initialization per Worker

wasm-bindgen emits a handful of hidden per-thread init and destroy shims (`__wbindgen_thread_destroy`, etc.) that must be called in every Worker that hosts a Rust thread. `wasm-bindgen-rayon` does this automatically; a hand-rolled pool that forgets to call the init shims will produce seemingly-random UB (double-frees, corrupt panic messages) because `#[thread_local]` statics will alias across threads.

### 6.5 Blocking on the main thread

Calling `Atomics.wait` on the main thread throws. Rust code that compiles fine but locks a contended `Mutex` or joins a thread from the main thread will therefore panic at runtime — or worse, if the blocking is inside a tight loop, visibly hang the tab. Rules of thumb: never hold a contended lock across a `.await` on the main thread; never join a worker thread from the main thread; push blocking work into a "coordinator" Worker.

### 6.6 Component Model threading is not ready

The [shared-everything threads](https://github.com/WebAssembly/shared-everything-threads) proposal is the Component Model's answer to threading. As of early 2026 it remains an **early-stage proposal (draft status on GitHub)**, with no shipping browser implementation and only early prototypes in wasmtime. WASI 0.3's roadmap mentions threading but the timeline is uncertain. Don't plan production code around this yet; stick to core-wasm + `wasm-bindgen` for browser threading.

### 6.7 `wasm-opt` and thread features

The Binaryen toolchain's `wasm-opt` (invoked by `wasm-pack --release`) must be passed `--enable-threads --enable-bulk-memory --enable-mutable-globals` or it will refuse to process a threaded module. Modern `wasm-pack` versions do this for you, but custom pipelines have hit this.

### 6.8 Debugging and stack traces

Cross-worker stack traces in DevTools are improving but still incomplete: a panic in a pool worker typically surfaces as a console error in that worker's context, not the main thread's. Install a panic hook early (`console_error_panic_hook` in each worker) or you will see silent aborts.

## 7. Recommendations by use case

| Use case | Recommended stack | Notes |
| --- | --- | --- |
| Image / video / audio processing, compression, ML inference | `wasm-bindgen-rayon` on `wasm32-unknown-unknown` with `+atomics` | Mature, used by Squoosh and Photoshop Web; `par_iter` and `ThreadPoolBuilder` work out of the box. |
| General data parallelism in Rust | `wasm-bindgen-rayon` (fork/join, `par_iter`) or `wasm_thread` (custom thread topology) | Pick rayon for pool-style, `wasm_thread` for a handful of long-lived specialist threads. |
| Async I/O (`fetch`, WebSockets, timers) | `wasm-bindgen-futures` + `tokio` `current_thread` runtime | Do not pursue real parallelism through `async` alone; browsers don't give you one. Pair with a thread pool if you also need CPU work. |
| Mixed async + CPU-parallel | `wasm-bindgen-futures` on main thread; offload heavy work to a `wasm-bindgen-rayon` pool via a small RPC layer (Comlink) | Keep the main thread event loop snappy; use `Atomics.waitAsync` under the hood for cross-thread wakeups. |
| Shipping a library to third-party sites | Prefer a single-threaded build, possibly with a COOP/COEP-gated faster path | COOP/COEP requirements often rule out embedding entirely; a fallback that works without shared memory is mandatory. |
| Targeting the Component Model today | Stick to core-wasm + `wasm-bindgen` if threads matter in the browser | `wasm32-wasip2` + `jco` is fine for non-threaded components; don't expect threads to work in-browser yet. |
| WASI server-side threading (wasmtime, etc.) | `wasm32-wasip1-threads` with `std::thread` | Not a browser target; included here for completeness. |
| UI frameworks (Leptos, Yew, Dioxus) | Usually single-threaded is fine; add `wasm-bindgen-rayon` only for a specific heavy feature | Framework-wide threading adds a lot of complexity for usually-small wins. |

## References

### Specifications

- [WebAssembly threads proposal](https://github.com/WebAssembly/threads) — the core proposal, now Phase 4.
- [WebAssembly shared-everything threads proposal](https://github.com/WebAssembly/shared-everything-threads) — Component-Model-level threading, early-stage draft proposal as of early 2026.
- [ECMA-262: Agents and `[[CanBlock]]`](https://tc39.es/ecma262/#sec-agents) — why `Atomics.wait` throws on the main thread.

### Rust docs and targets

- [Rust platform support](https://doc.rust-lang.org/rustc/platform-support.html) — target tier list (`wasm32-unknown-unknown`, `wasm32-wasip1-threads`, `wasm32-wasip2`).
- [`wasm-bindgen` guide: raytrace with threads](https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html) — the canonical example.
- [The `rustwasm` book](https://rustwasm.github.io/docs/book/) — broader introduction to Rust/WASM.

### Browser APIs

- [MDN: `SharedArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SharedArrayBuffer)
- [MDN: `Atomics`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics)
- [MDN: `Atomics.wait`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics/wait)
- [MDN: `Atomics.waitAsync`](https://developer.mozilla.org/en-US/docs/Web/API/Atomics/waitAsync)
- [MDN: `Worker`](https://developer.mozilla.org/en-US/docs/Web/API/Worker)
- [MDN: `WebAssembly.Memory`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Memory)
- [web.dev: Making your website cross-origin isolated (COOP/COEP)](https://web.dev/articles/coop-coep)
- [MDN: `Cross-Origin-Embedder-Policy` (including `credentialless`)](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy)
- [caniuse: SharedArrayBuffer](https://caniuse.com/sharedarraybuffer)
- [Chrome blog: WebAssembly threads](https://developer.chrome.com/blog/wasm-threads)

### Libraries

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
- [`wasm-bindgen-rayon`](https://github.com/RReverser/wasm-bindgen-rayon)
- [`wasm_thread` on crates.io](https://crates.io/crates/wasm_thread)
- [`rayon`](https://github.com/rayon-rs/rayon)
- [`parking_lot`](https://github.com/Amanieu/parking_lot)
- [`crossbeam`](https://github.com/crossbeam-rs/crossbeam)
- [`tokio`](https://github.com/tokio-rs/tokio) — for the `current_thread` runtime discussion.
- [`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures)
- [Comlink](https://github.com/GoogleChromeLabs/comlink)
- [`vite-plugin-cross-origin-isolation`](https://github.com/chaosprint/vite-plugin-cross-origin-isolation)

### Case studies and deployments

- [Squoosh](https://squoosh.app/) and its [source](https://github.com/GoogleChromeLabs/squoosh) — reference consumer of `wasm-bindgen-rayon`.
- [Adobe Photoshop on the Web: engineering overview](https://web.dev/articles/ps-on-the-web) — large-scale threaded WASM in production.
- [Google Meet WASM audio/video](https://web.dev/case-studies/google-meet-wasm) — real-time threaded codecs.
- [wasm-bindgen-rayon README: getting started](https://github.com/RReverser/wasm-bindgen-rayon#readme) — the copy-pasteable recipe.

### Known limitations of this document

Remaining uncertainties:

- Current Rust tier status of `wasm32-wasip2` — stated as experimental for threading; confirm against the platform support page at time of reading.
- Whether the shared-everything-threads proposal has advanced in formal phase status since early 2026 — stated as an early-stage draft, but the proposal moves.
- Performance numbers are deliberately qualitative; no concrete microbenchmark citations are included. For engineering decisions, measure on your target workload and browsers.
