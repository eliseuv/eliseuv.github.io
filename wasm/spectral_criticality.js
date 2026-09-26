/**
 * Time series matrices of one source at one parameter point, generated run after run, with
 * the spectra of their correlation matrices accumulated.
 *
 * The time series matrix is the run in progress; the correlation matrix and its eigenvalues
 * are those of the last completed run.
 */
export class SpectralLab {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SpectralLab.prototype);
        obj.__wbg_ptr = ptr;
        SpectralLabFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpectralLabFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_spectrallab_free(ptr, 0);
    }
    /**
     * Eigenvalues of every completed run, pointer to `n_matrices() * n_samples()` `f64`.
     * @returns {number}
     */
    accumulated_eigenvalues() {
        const ret = wasm.spectrallab_accumulated_eigenvalues(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Advance by `steps` time steps, completing and analysing runs (and starting new ones) as
     * needed. Returns the number of runs completed.
     * @param {number} steps
     * @returns {number}
     */
    advance(steps) {
        const ret = wasm.spectrallab_advance(this.__wbg_ptr, steps);
        return ret >>> 0;
    }
    /**
     * Density series of the one-dimensional contact process with infection rate `alpha` and
     * diffusion probability `gamma`, from fully active chains.
     * @param {number} alpha
     * @param {number} gamma
     * @returns {SpectralLab}
     */
    static contact_process(alpha, gamma) {
        const ret = wasm.spectrallab_contact_process(alpha, gamma);
        return SpectralLab.__wrap(ret);
    }
    /**
     * Gaussian series whose consecutive pairs have correlation `rho`.
     * @param {number} rho
     * @returns {SpectralLab}
     */
    static correlated_pairs(rho) {
        const ret = wasm.spectrallab_correlated_pairs(rho);
        return SpectralLab.__wrap(ret);
    }
    /**
     * Counts of the off-diagonal correlations of every completed run over
     * [`reference::N_BINS`] equal bins of `[-1, 1]`.
     * @returns {Float64Array}
     */
    correlation_counts() {
        const ret = wasm.spectrallab_correlation_counts(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * Correlation matrix of the last completed run, `n_samples × n_samples` row-major `f64`.
     * @returns {number}
     */
    correlations() {
        const ret = wasm.spectrallab_correlations(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    eigenvalue_variance() {
        const ret = wasm.spectrallab_eigenvalue_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * Eigenvalues (ascending) of the last completed run; empty before the first one.
     * @returns {Float64Array}
     */
    eigenvalues() {
        const ret = wasm.spectrallab_eigenvalues(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * Magnetization series of the square lattice Ising model at `t_over_tc` times the Onsager
     * temperature, from random configurations.
     * @param {number} t_over_tc
     * @returns {SpectralLab}
     */
    static ising(t_over_tc) {
        const ret = wasm.spectrallab_ising(t_over_tc);
        return SpectralLab.__wrap(ret);
    }
    /**
     * Lattice columns (`0` for sources without a lattice).
     * @returns {number}
     */
    lattice_cols() {
        const ret = wasm.spectrallab_lattice_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Lattice rows (`0` for sources without a lattice).
     * @returns {number}
     */
    lattice_rows() {
        const ret = wasm.spectrallab_lattice_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    max_eigenvalue_mean() {
        const ret = wasm.spectrallab_max_eigenvalue_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    max_eigenvalue_variance() {
        const ret = wasm.spectrallab_max_eigenvalue_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * Completed runs, i.e. correlation matrices accumulated.
     * @returns {bigint}
     */
    n_matrices() {
        const ret = wasm.spectrallab_n_matrices(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {number}
     */
    n_samples() {
        const ret = wasm.spectrallab_n_samples(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Measurements per series after the initial one.
     * @returns {number}
     */
    n_steps() {
        const ret = wasm.spectrallab_n_steps(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Time series matrix of the run in progress, `n_samples × (n_steps + 1)` row-major `f64`;
     * columns after `t` are zero.
     * @returns {number}
     */
    series() {
        const ret = wasm.spectrallab_series(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Sites of sample `sample` (one byte each: `i8` spins, `u8` activity), or null for
     * sources without a lattice.
     * @param {number} sample
     * @returns {number}
     */
    sites(sample) {
        const ret = wasm.spectrallab_sites(this.__wbg_ptr, sample);
        return ret >>> 0;
    }
    /**
     * Last filled time step of the run in progress.
     * @returns {number}
     */
    t() {
        const ret = wasm.spectrallab_t(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Uncorrelated Gaussian series: the Marchenko-Pastur baseline.
     * @returns {SpectralLab}
     */
    static white_noise() {
        const ret = wasm.spectrallab_white_noise();
        return SpectralLab.__wrap(ret);
    }
}
if (Symbol.dispose) SpectralLab.prototype[Symbol.dispose] = SpectralLab.prototype.free;

/**
 * Best estimate of the critical infection rate of the one-dimensional contact process.
 * @returns {number}
 */
export function contact_process_alpha_critical() {
    const ret = wasm.contact_process_alpha_critical();
    return ret;
}

/**
 * Decode gzipped CBOR reference scans into JSON.
 * @param {Uint8Array} gzipped_cbor
 * @returns {string}
 */
export function decode_reference(gzipped_cbor) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passArray8ToWasm0(gzipped_cbor, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.decode_reference(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * Critical temperature of the infinite square lattice Ising model (Onsager).
 * @returns {number}
 */
export function ising_t_critical() {
    const ret = wasm.ising_t_critical();
    return ret;
}

/**
 * Marchenko-Pastur density of `n_samples` uncorrelated series of `n_steps + 1` values.
 * @param {number} n_steps
 * @param {number} lambda
 * @returns {number}
 */
export function marchenko_pastur_density(n_steps, lambda) {
    const ret = wasm.marchenko_pastur_density(n_steps, lambda);
    return ret;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_83742b46f01ce22d: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
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
        __wbg_getRandomValues_cc7f052a444bb2ce: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
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
        "./spectral_criticality_bg.js": import0,
    };
}

const SpectralLabFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_spectrallab_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
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

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
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

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
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
        module_or_path = new URL('spectral_criticality_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
