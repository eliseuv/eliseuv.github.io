/**
 * wasm-bindgen binding around `annealing::tsp`, wiring its
 * RNG closure to `js_sys::Math::random`.
 */
export class TspSimulation {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TspSimulation.prototype);
        obj.__wbg_ptr = ptr;
        TspSimulationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TspSimulationFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tspsimulation_free(ptr, 0);
    }
    /**
     * Run `n_moves` Metropolis iterations.
     * @param {number} n_moves
     */
    advance(n_moves) {
        wasm.tspsimulation_advance(this.__wbg_ptr, n_moves);
    }
    /**
     * Restart from a random cycle at `T₀` on the same cities.
     */
    anneal() {
        wasm.tspsimulation_anneal(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    annealing_step() {
        const ret = wasm.tspsimulation_annealing_step(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    best_cost() {
        const ret = wasm.tspsimulation_best_cost(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    city_count() {
        const ret = wasm.tspsimulation_city_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cost() {
        const ret = wasm.tspsimulation_cost(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    expected_random_cost() {
        const ret = wasm.tspsimulation_expected_random_cost(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    is_frozen() {
        const ret = wasm.tspsimulation_is_frozen(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * `n` cities from `domain` (see `randomize`), annealed from a random
     * cycle at `initial_temperature`.
     * @param {number} n
     * @param {string} domain
     * @param {number} rho
     * @param {number} gamma
     * @param {number} x0
     * @param {string} move_kind
     * @param {number} initial_temperature
     * @param {number} final_temperature
     * @param {number} alpha
     * @param {number} iterations_per_step
     * @returns {TspSimulation}
     */
    static new(n, domain, rho, gamma, x0, move_kind, initial_temperature, final_temperature, alpha, iterations_per_step) {
        const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(move_kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.tspsimulation_new(n, ptr0, len0, rho, gamma, x0, ptr1, len1, initial_temperature, final_temperature, alpha, iterations_per_step);
        return TspSimulation.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    points_x() {
        const ret = wasm.tspsimulation_points_x(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    points_y() {
        const ret = wasm.tspsimulation_points_y(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * New cities, then `anneal`. `domain` is one of `"uniform"`,
     * `"correlated-uniform"`, `"correlated-normal"` (using `rho`) or
     * `"power-law"` (using `gamma` and `x0`).
     * @param {number} n
     * @param {string} domain
     * @param {number} rho
     * @param {number} gamma
     * @param {number} x0
     */
    randomize(n, domain, rho, gamma, x0) {
        const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.tspsimulation_randomize(this.__wbg_ptr, n, ptr0, len0, rho, gamma, x0);
    }
    /**
     * @param {number} alpha
     */
    set_alpha(alpha) {
        wasm.tspsimulation_set_alpha(this.__wbg_ptr, alpha);
    }
    /**
     * @param {number} temperature
     */
    set_final_temperature(temperature) {
        wasm.tspsimulation_set_final_temperature(this.__wbg_ptr, temperature);
    }
    /**
     * @param {boolean} hold
     */
    set_hold(hold) {
        wasm.tspsimulation_set_hold(this.__wbg_ptr, hold);
    }
    /**
     * @param {number} temperature
     */
    set_initial_temperature(temperature) {
        wasm.tspsimulation_set_initial_temperature(this.__wbg_ptr, temperature);
    }
    /**
     * @param {number} iterations
     */
    set_iterations_per_step(iterations) {
        wasm.tspsimulation_set_iterations_per_step(this.__wbg_ptr, iterations);
    }
    /**
     * `"swap"` or `"2-opt"`.
     * @param {string} move_kind
     */
    set_move(move_kind) {
        const ptr0 = passStringToWasm0(move_kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.tspsimulation_set_move(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Manual override of the current temperature.
     * @param {number} temperature
     */
    set_temperature(temperature) {
        wasm.tspsimulation_set_temperature(this.__wbg_ptr, temperature);
    }
    /**
     * Fraction of moves accepted since the previous call.
     * @returns {number}
     */
    take_acceptance_rate() {
        const ret = wasm.tspsimulation_take_acceptance_rate(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    temperature() {
        const ret = wasm.tspsimulation_temperature(this.__wbg_ptr);
        return ret;
    }
    /**
     * City indices in visiting order.
     * @returns {Uint32Array}
     */
    tour() {
        const ret = wasm.tspsimulation_tour(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}
if (Symbol.dispose) TspSimulation.prototype[Symbol.dispose] = TspSimulation.prototype.free;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_random_5bb86cae65a45bf6: function() {
            const ret = Math.random();
            return ret;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./tsp_annealing_bg.js": import0,
    };
}

const TspSimulationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tspsimulation_free(ptr >>> 0, 1));

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('tsp_annealing_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
