(function() {
    var wasm;
    const __exports = {};
    /**
    * @returns {void}
    */
    __exports.run = function() {
        return wasm.run();
    };

    const __widl_f_set_property_CSSStyleDeclaration_target = typeof CSSStyleDeclaration === 'undefined' ? null : CSSStyleDeclaration.prototype.setProperty || function() {
        throw new Error(`wasm-bindgen: CSSStyleDeclaration.setProperty does not exist`);
    };

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }

    const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

    let slab_next = slab.length;

    function addHeapObject(obj) {
        if (slab_next === slab.length) slab.push(slab.length + 1);
        const idx = slab_next;
        const next = slab[idx];

        slab_next = next;

        slab[idx] = { obj, cnt: 1 };
        return idx << 1;
    }

    const stack = [];

    function getObject(idx) {
        if ((idx & 1) === 1) {
            return stack[idx >> 1];
        } else {
            const val = slab[idx >> 1];

            return val.obj;

        }
    }

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

    __exports.__widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        let varg3 = getStringFromWasm(arg3, arg4);
        try {
            __widl_f_set_property_CSSStyleDeclaration_target.call(getObject(arg0), varg1, varg3);
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_create_element_Document_target = typeof Document === 'undefined' ? null : Document.prototype.createElement || function() {
        throw new Error(`wasm-bindgen: Document.createElement does not exist`);
    };

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

    const __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {
        throw new Error(`wasm-bindgen: Document.getElementById does not exist`);
    };

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    __exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);

        const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

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

let cachedTextEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

__exports.__widl_f_ready_state_Document = function(ret, arg0) {

    const [retptr, retlen] = passStringToWasm(__widl_f_ready_state_Document_target.call(getObject(arg0)));
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_body_Document_target = GetOwnOrInheritedPropertyDescriptor(typeof Document === 'undefined' ? null : Document.prototype, 'body').get || function() {
    throw new Error(`wasm-bindgen: Document.body does not exist`);
};

__exports.__widl_f_body_Document = function(arg0) {

    const val = __widl_f_body_Document_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_set_onload_Document_target = GetOwnOrInheritedPropertyDescriptor(typeof Document === 'undefined' ? null : Document.prototype, 'onload').set || function() {
    throw new Error(`wasm-bindgen: Document.onload does not exist`);
};

__exports.__widl_f_set_onload_Document = function(arg0, arg1) {
    __widl_f_set_onload_Document_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_set_id_Element_target = GetOwnOrInheritedPropertyDescriptor(typeof Element === 'undefined' ? null : Element.prototype, 'id').set || function() {
    throw new Error(`wasm-bindgen: Element.id does not exist`);
};

__exports.__widl_f_set_id_Element = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_id_Element_target.call(getObject(arg0), varg1);
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

const __widl_f_set_type_HTMLButtonElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLButtonElement === 'undefined' ? null : HTMLButtonElement.prototype, 'type').set || function() {
    throw new Error(`wasm-bindgen: HTMLButtonElement.type does not exist`);
};

__exports.__widl_f_set_type_HTMLButtonElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_type_HTMLButtonElement_target.call(getObject(arg0), varg1);
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

const __widl_f_set_inner_text_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'innerText').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.innerText does not exist`);
};

__exports.__widl_f_set_inner_text_HTMLElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_inner_text_HTMLElement_target.call(getObject(arg0), varg1);
};

const __widl_f_style_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'style').get || function() {
    throw new Error(`wasm-bindgen: HTMLElement.style does not exist`);
};

__exports.__widl_f_style_HTMLElement = function(arg0) {
    return addHeapObject(__widl_f_style_HTMLElement_target.call(getObject(arg0)));
};

const __widl_f_set_onsubmit_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'onsubmit').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.onsubmit does not exist`);
};

__exports.__widl_f_set_onsubmit_HTMLElement = function(arg0, arg1) {
    __widl_f_set_onsubmit_HTMLElement_target.call(getObject(arg0), getObject(arg1));
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

const __widl_f_set_max_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'max').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.max does not exist`);
};

__exports.__widl_f_set_max_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_max_HTMLInputElement_target.call(getObject(arg0), varg1);
};

const __widl_f_set_min_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'min').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.min does not exist`);
};

__exports.__widl_f_set_min_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_min_HTMLInputElement_target.call(getObject(arg0), varg1);
};

const __widl_f_set_name_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'name').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.name does not exist`);
};

__exports.__widl_f_set_name_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_name_HTMLInputElement_target.call(getObject(arg0), varg1);
};

const __widl_f_set_required_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'required').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.required does not exist`);
};

__exports.__widl_f_set_required_HTMLInputElement = function(arg0, arg1) {
    __widl_f_set_required_HTMLInputElement_target.call(getObject(arg0), arg1 !== 0);
};

const __widl_f_set_type_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'type').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.type does not exist`);
};

__exports.__widl_f_set_type_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_type_HTMLInputElement_target.call(getObject(arg0), varg1);
};

const __widl_f_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'value').get || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.value does not exist`);
};

__exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

    const [retptr, retlen] = passStringToWasm(__widl_f_value_HTMLInputElement_target.call(getObject(arg0)));
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_set_value_as_number_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'valueAsNumber').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.valueAsNumber does not exist`);
};

__exports.__widl_f_set_value_as_number_HTMLInputElement = function(arg0, arg1) {
    __widl_f_set_value_as_number_HTMLInputElement_target.call(getObject(arg0), arg1);
};

__exports.__widl_instanceof_HTMLSpanElement = function(idx) {
    return getObject(idx) instanceof HTMLSpanElement ? 1 : 0;
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

const __widl_f_set_border_HTMLTableElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLTableElement === 'undefined' ? null : HTMLTableElement.prototype, 'border').set || function() {
    throw new Error(`wasm-bindgen: HTMLTableElement.border does not exist`);
};

__exports.__widl_f_set_border_HTMLTableElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_border_HTMLTableElement_target.call(getObject(arg0), varg1);
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

__exports.__wbg_newnoargs_96cbdf0d056b2fa8 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_ee8306f6b79399de = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__wbindgen_object_clone_ref = function(idx) {
    // If this object is on the stack promote it to the heap.
    if ((idx & 1) === 1) return addHeapObject(getObject(idx));

    // Otherwise if the object is on the heap just bump the
    // refcount and move on
    const val = slab[idx >> 1];
    val.cnt += 1;
    return idx;
};

function dropRef(idx) {

    idx = idx >> 1;
    if (idx < 4) return;
    let obj = slab[idx];

    obj.cnt -= 1;
    if (obj.cnt > 0) return;

    // If we hit 0 then free up our space in the slab
    slab[idx] = slab_next;
    slab_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) {
    dropRef(i);
};

__exports.__wbindgen_number_new = function(i) {
    return addHeapObject(i);
};

__exports.__wbindgen_cb_forget = dropRef;

function takeObject(idx) {
    const ret = getObject(idx);
    dropRef(idx);
    return ret;
}

__exports.__wbindgen_rethrow = function(idx) { throw takeObject(idx); };

__exports.__wbindgen_closure_wrapper4 = function(a, b, fi, di, _ignored) {
    const f = wasm.__wbg_function_table.get(fi);
    const d = wasm.__wbg_function_table.get(di);
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

__exports.__wbindgen_closure_wrapper6 = function(a, b, fi, di, _ignored) {
    const f = wasm.__wbg_function_table.get(fi);
    const d = wasm.__wbg_function_table.get(di);
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
        return { instance, module: module_or_path }
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
    return;
});
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
