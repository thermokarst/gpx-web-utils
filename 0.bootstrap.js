(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/gpx_web_utils.js":
/*!*******************************!*\
  !*** ../pkg/gpx_web_utils.js ***!
  \*******************************/
/*! exports provided: greet, __wbg_alert_4ad63c9979edd105, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gpx_web_utils_bg.wasm */ \"../pkg/gpx_web_utils_bg.wasm\");\n/* harmony import */ var _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./gpx_web_utils_bg.js */ \"../pkg/gpx_web_utils_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"greet\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_4ad63c9979edd105\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_alert_4ad63c9979edd105\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _gpx_web_utils_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/gpx_web_utils.js?");

/***/ }),

/***/ "../pkg/gpx_web_utils_bg.js":
/*!**********************************!*\
  !*** ../pkg/gpx_web_utils_bg.js ***!
  \**********************************/
/*! exports provided: greet, __wbg_alert_4ad63c9979edd105, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_4ad63c9979edd105\", function() { return __wbg_alert_4ad63c9979edd105; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony import */ var _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gpx_web_utils_bg.wasm */ \"../pkg/gpx_web_utils_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction greet() {\n    _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst __wbg_alert_4ad63c9979edd105 = function(arg0, arg1) {\n    alert(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _gpx_web_utils_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/gpx_web_utils_bg.js?");

/***/ }),

/***/ "../pkg/gpx_web_utils_bg.wasm":
/*!************************************!*\
  !*** ../pkg/gpx_web_utils_bg.wasm ***!
  \************************************/
/*! exports provided: memory, greet, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./gpx_web_utils_bg.js */ \"../pkg/gpx_web_utils_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/gpx_web_utils_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var gpx_web_utils__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gpx-web-utils */ \"../pkg/gpx_web_utils.js\");\n\n\ngpx_web_utils__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);