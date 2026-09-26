/**
 * wasm-bindgen binding around `quant::chiarella`,
 * wiring its RNG closure to `js_sys::Math::random`.
 */
export class ChiarellaSimulation {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ChiarellaSimulation.prototype);
        obj.__wbg_ptr = ptr;
        ChiarellaSimulationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChiarellaSimulationFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chiarellasimulation_free(ptr, 0);
    }
    /**
     * Run `n_steps` Euler–Maruyama steps.
     * @param {number} n_steps
     */
    advance(n_steps) {
        wasm.chiarellasimulation_advance(this.__wbg_ptr, n_steps);
    }
    /**
     * @returns {number}
     */
    fundamental() {
        const ret = wasm.chiarellasimulation_fundamental(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    is_oscillatory() {
        const ret = wasm.chiarellasimulation_is_oscillatory(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    mispricing() {
        const ret = wasm.chiarellasimulation_mispricing(this.__wbg_ptr);
        return ret;
    }
    /**
     * New simulation at `p = V = M = 0`.
     * @param {number} kappa
     * @param {number} beta
     * @param {number} gamma
     * @param {number} alpha
     * @param {number} sigma_noise
     * @param {number} sigma_fundamental
     * @param {number} drift
     * @param {number} dt
     * @returns {ChiarellaSimulation}
     */
    static new(kappa, beta, gamma, alpha, sigma_noise, sigma_fundamental, drift, dt) {
        const ret = wasm.chiarellasimulation_new(kappa, beta, gamma, alpha, sigma_noise, sigma_fundamental, drift, dt);
        return ChiarellaSimulation.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    price() {
        const ret = wasm.chiarellasimulation_price(this.__wbg_ptr);
        return ret;
    }
    /**
     * Reset to `p = V = M = 0`, keeping parameters.
     */
    reset() {
        wasm.chiarellasimulation_reset(this.__wbg_ptr);
    }
    /**
     * @param {number} alpha
     */
    set_alpha(alpha) {
        wasm.chiarellasimulation_set_alpha(this.__wbg_ptr, alpha);
    }
    /**
     * @param {number} beta
     */
    set_beta(beta) {
        wasm.chiarellasimulation_set_beta(this.__wbg_ptr, beta);
    }
    /**
     * @param {number} drift
     */
    set_drift(drift) {
        wasm.chiarellasimulation_set_drift(this.__wbg_ptr, drift);
    }
    /**
     * @param {number} dt
     */
    set_dt(dt) {
        wasm.chiarellasimulation_set_dt(this.__wbg_ptr, dt);
    }
    /**
     * @param {number} gamma
     */
    set_gamma(gamma) {
        wasm.chiarellasimulation_set_gamma(this.__wbg_ptr, gamma);
    }
    /**
     * @param {number} kappa
     */
    set_kappa(kappa) {
        wasm.chiarellasimulation_set_kappa(this.__wbg_ptr, kappa);
    }
    /**
     * @param {number} sigma_fundamental
     */
    set_sigma_fundamental(sigma_fundamental) {
        wasm.chiarellasimulation_set_sigma_fundamental(this.__wbg_ptr, sigma_fundamental);
    }
    /**
     * @param {number} sigma_noise
     */
    set_sigma_noise(sigma_noise) {
        wasm.chiarellasimulation_set_sigma_noise(this.__wbg_ptr, sigma_noise);
    }
    /**
     * @returns {number}
     */
    trend() {
        const ret = wasm.chiarellasimulation_trend(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) ChiarellaSimulation.prototype[Symbol.dispose] = ChiarellaSimulation.prototype.free;

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
        "./chiarella_bg.js": import0,
    };
}

const ChiarellaSimulationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chiarellasimulation_free(ptr >>> 0, 1));

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
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
        module_or_path = new URL('chiarella_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
