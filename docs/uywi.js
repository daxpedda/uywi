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
    __exports.onsubmit = function(arg0) {
        try {
            return wasm.onsubmit(addBorrowedObject(arg0));

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

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const __widl_f_create_element_Document_target = typeof Document === 'undefined' ? null : Document.prototype.createElement || function() {
    throw new Error(`wasm-bindgen: Document.createElement does not exist`);
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

__exports.__widl_f_create_element_Document = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(__widl_f_create_element_Document_target.call(getObject(arg0), varg1));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

const __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {
    throw new Error(`wasm-bindgen: Document.getElementById does not exist`);
};

__exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);
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

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
return {}
}

const __widl_f_ready_state_Document_target = GetOwnOrInheritedPropertyDescriptor(typeof Document === 'undefined' ? null : Document.prototype, 'readyState').get || function() {
    throw new Error(`wasm-bindgen: Document.readyState does not exist`);
};

__exports.__widl_f_ready_state_Document = function(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_ready_state_Document_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_forms_Document_target = GetOwnOrInheritedPropertyDescriptor(typeof Document === 'undefined' ? null : Document.prototype, 'forms').get || function() {
    throw new Error(`wasm-bindgen: Document.forms does not exist`);
};

__exports.__widl_f_forms_Document = function(arg0) {
    return addHeapObject(__widl_f_forms_Document_target.call(getObject(arg0)));
};

const __widl_f_set_onload_Document_target = GetOwnOrInheritedPropertyDescriptor(typeof Document === 'undefined' ? null : Document.prototype, 'onload').set || function() {
    throw new Error(`wasm-bindgen: Document.onload does not exist`);
};

__exports.__widl_f_set_onload_Document = function(arg0, arg1) {
    __widl_f_set_onload_Document_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_prevent_default_Event_target = typeof Event === 'undefined' ? null : Event.prototype.preventDefault || function() {
    throw new Error(`wasm-bindgen: Event.preventDefault does not exist`);
};

__exports.__widl_f_prevent_default_Event = function(arg0) {
    __widl_f_prevent_default_Event_target.call(getObject(arg0));
};

const __widl_f_target_Event_target = GetOwnOrInheritedPropertyDescriptor(typeof Event === 'undefined' ? null : Event.prototype, 'target').get || function() {
    throw new Error(`wasm-bindgen: Event.target does not exist`);
};

__exports.__widl_f_target_Event = function(arg0) {

    const val = __widl_f_target_Event_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_instanceof_HTMLButtonElement = function(idx) {
    return getObject(idx) instanceof HTMLButtonElement ? 1 : 0;
};

__exports.__widl_f_get_with_index_HTMLCollection = function(arg0, arg1) {

    const val = getObject(arg0)[arg1];
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_length_HTMLCollection_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCollection === 'undefined' ? null : HTMLCollection.prototype, 'length').get || function() {
    throw new Error(`wasm-bindgen: HTMLCollection.length does not exist`);
};

__exports.__widl_f_length_HTMLCollection = function(arg0) {
    return __widl_f_length_HTMLCollection_target.call(getObject(arg0));
};

const __widl_f_click_HTMLElement_target = typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype.click || function() {
    throw new Error(`wasm-bindgen: HTMLElement.click does not exist`);
};

__exports.__widl_f_click_HTMLElement = function(arg0) {
    __widl_f_click_HTMLElement_target.call(getObject(arg0));
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

const __widl_f_value_as_number_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'valueAsNumber').get || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.valueAsNumber does not exist`);
};

__exports.__widl_f_value_as_number_HTMLInputElement = function(arg0) {
    return __widl_f_value_as_number_HTMLInputElement_target.call(getObject(arg0));
};

__exports.__widl_instanceof_HTMLLinkElement = function(idx) {
    return getObject(idx) instanceof HTMLLinkElement ? 1 : 0;
};

__exports.__widl_instanceof_HTMLTableElement = function(idx) {
    return getObject(idx) instanceof HTMLTableElement ? 1 : 0;
};

const __widl_f_delete_row_HTMLTableElement_target = typeof HTMLTableElement === 'undefined' ? null : HTMLTableElement.prototype.deleteRow || function() {
    throw new Error(`wasm-bindgen: HTMLTableElement.deleteRow does not exist`);
};

__exports.__widl_f_delete_row_HTMLTableElement = function(arg0, arg1, exnptr) {
    try {
        __widl_f_delete_row_HTMLTableElement_target.call(getObject(arg0), arg1);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_insert_row_HTMLTableElement_target = typeof HTMLTableElement === 'undefined' ? null : HTMLTableElement.prototype.insertRow || function() {
    throw new Error(`wasm-bindgen: HTMLTableElement.insertRow does not exist`);
};

__exports.__widl_f_insert_row_HTMLTableElement = function(arg0, exnptr) {
    try {
        return addHeapObject(__widl_f_insert_row_HTMLTableElement_target.call(getObject(arg0)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_rows_HTMLTableElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLTableElement === 'undefined' ? null : HTMLTableElement.prototype, 'rows').get || function() {
    throw new Error(`wasm-bindgen: HTMLTableElement.rows does not exist`);
};

__exports.__widl_f_rows_HTMLTableElement = function(arg0) {
    return addHeapObject(__widl_f_rows_HTMLTableElement_target.call(getObject(arg0)));
};

__exports.__widl_instanceof_HTMLTableRowElement = function(idx) {
    return getObject(idx) instanceof HTMLTableRowElement ? 1 : 0;
};

const __widl_f_insert_cell_HTMLTableRowElement_target = typeof HTMLTableRowElement === 'undefined' ? null : HTMLTableRowElement.prototype.insertCell || function() {
    throw new Error(`wasm-bindgen: HTMLTableRowElement.insertCell does not exist`);
};

__exports.__widl_f_insert_cell_HTMLTableRowElement = function(arg0, exnptr) {
    try {
        return addHeapObject(__widl_f_insert_cell_HTMLTableRowElement_target.call(getObject(arg0)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_append_child_Node_target = typeof Node === 'undefined' ? null : Node.prototype.appendChild || function() {
    throw new Error(`wasm-bindgen: Node.appendChild does not exist`);
};

__exports.__widl_f_append_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(__widl_f_append_child_Node_target.call(getObject(arg0), getObject(arg1)));
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

__exports.__wbg_newnoargs_6a80f84471205fc8 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_582b20dfcad7fee4 = function(arg0, arg1, exnptr) {
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

__exports.__wbindgen_closure_wrapper7 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(2);
    const d = wasm.__wbg_function_table.get(3);
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
