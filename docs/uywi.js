(function() {
    var wasm;
    const __exports = {};
    /**
    * @returns {void}
    */
    __exports.run = function() {
        return wasm.run();
    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    /**
    * @param {any} arg0
    * @returns {void}
    */
    __exports.display_page = function(arg0) {
        try {
            return wasm.display_page(addBorrowedObject(arg0));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    };

    /**
    * @param {any} arg0
    * @returns {void}
    */
    __exports.display_concept_by_concept = function(arg0) {
        try {
            return wasm.display_concept_by_concept(addBorrowedObject(arg0));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    };

    /**
    * @param {any} arg0
    * @returns {void}
    */
    __exports.display_concept_by_index = function(arg0) {
        try {
            return wasm.display_concept_by_index(addBorrowedObject(arg0));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    };

    /**
    * @param {any} arg0
    * @returns {void}
    */
    __exports.check_concept = function(arg0) {
        try {
            return wasm.check_concept(addBorrowedObject(arg0));

        } finally {
            heap[stack_pointer++] = undefined;

        }

    };

function getObject(idx) { return heap[idx]; }

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

__exports.__widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setProperty(varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_create_element_Document = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(getObject(arg0).createElement(varg1));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

__exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}

__exports.__widl_f_ready_state_Document = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).readyState);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_forms_Document = function(arg0) {
    return addHeapObject(getObject(arg0).forms);
};

__exports.__widl_f_set_onload_Document = function(arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
};

__exports.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setAttribute(varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_prevent_default_Event = function(arg0) {
    getObject(arg0).preventDefault();
};

__exports.__widl_f_target_Event = function(arg0) {

    const val = getObject(arg0).target;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_instanceof_HTMLButtonElement = function(idx) {
    return getObject(idx) instanceof HTMLButtonElement ? 1 : 0;
};

__exports.__widl_f_get_with_index_HTMLCollection = function(arg0, arg1) {

    const val = getObject(arg0)[arg1];
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_length_HTMLCollection = function(arg0) {
    return getObject(arg0).length;
};

__exports.__widl_instanceof_HTMLElement = function(idx) {
    return getObject(idx) instanceof HTMLElement ? 1 : 0;
};

__exports.__widl_f_click_HTMLElement = function(arg0) {
    getObject(arg0).click();
};

__exports.__widl_f_set_inner_text_HTMLElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).innerText = varg1;
};

__exports.__widl_f_style_HTMLElement = function(arg0) {
    return addHeapObject(getObject(arg0).style);
};

__exports.__widl_f_set_onclick_HTMLElement = function(arg0, arg1) {
    getObject(arg0).onclick = getObject(arg1);
};

__exports.__widl_instanceof_HTMLFormElement = function(idx) {
    return getObject(idx) instanceof HTMLFormElement ? 1 : 0;
};

__exports.__widl_f_get_with_name_HTMLFormElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(getObject(arg0)[varg1]);
};

__exports.__widl_instanceof_HTMLInputElement = function(idx) {
    return getObject(idx) instanceof HTMLInputElement ? 1 : 0;
};

__exports.__widl_f_check_validity_HTMLInputElement = function(arg0) {
    return getObject(arg0).checkValidity();
};

__exports.__widl_f_set_custom_validity_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).setCustomValidity(varg1);
};

__exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).value);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).value = varg1;
};

__exports.__widl_f_value_as_number_HTMLInputElement = function(arg0) {
    return getObject(arg0).valueAsNumber;
};

__exports.__widl_f_set_value_as_number_HTMLInputElement = function(arg0, arg1) {
    getObject(arg0).valueAsNumber = arg1;
};

__exports.__widl_instanceof_HTMLTableElement = function(idx) {
    return getObject(idx) instanceof HTMLTableElement ? 1 : 0;
};

__exports.__widl_f_delete_row_HTMLTableElement = function(arg0, arg1, exnptr) {
    try {
        getObject(arg0).deleteRow(arg1);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_insert_row_HTMLTableElement = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).insertRow());
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_rows_HTMLTableElement = function(arg0) {
    return addHeapObject(getObject(arg0).rows);
};

__exports.__widl_instanceof_HTMLTableRowElement = function(idx) {
    return getObject(idx) instanceof HTMLTableRowElement ? 1 : 0;
};

__exports.__widl_f_insert_cell_HTMLTableRowElement = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).insertCell());
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_append_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_instanceof_Window = function(idx) {
    return getObject(idx) instanceof Window ? 1 : 0;
};

__exports.__widl_f_document_Window = function(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__wbg_newnoargs_970ffcd96c15d34e = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_6ecd167e59b01396 = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__wbindgen_object_clone_ref = function(idx) {
    return addHeapObject(getObject(idx));
};

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

__exports.__wbindgen_string_new = function(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
};

__exports.__wbindgen_cb_forget = dropObject;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

__exports.__wbindgen_rethrow = function(idx) { throw takeObject(idx); };

__exports.__wbindgen_closure_wrapper26 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(7);
    const d = wasm.__wbg_function_table.get(8);
    const cb = function() {
        this.cnt++;
        try {
            return f(this.a, b);

        } finally {
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_closure_wrapper28 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(11);
    const d = wasm.__wbg_function_table.get(12);
    const cb = function(arg0) {
        this.cnt++;
        try {
            return f(this.a, b, addHeapObject(arg0));

        } finally {
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function init(path_or_module) {
    let instantiation;
    const imports = { './uywi': __exports };
    if (path_or_module instanceof WebAssembly.Module) {
        instantiation = WebAssembly.instantiate(path_or_module, imports)
        .then(instance => {
        return { instance, module: path_or_module }
    });
} else {
    const data = fetch(path_or_module);
    if (typeof WebAssembly.instantiateStreaming === 'function') {
        instantiation = WebAssembly.instantiateStreaming(data, imports);
    } else {
        instantiation = data
        .then(response => response.arrayBuffer())
        .then(buffer => WebAssembly.instantiate(buffer, imports));
    }
}
return instantiation.then(({instance}) => {
    wasm = init.wasm = instance.exports;
    wasm.__wbindgen_start();
});
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
