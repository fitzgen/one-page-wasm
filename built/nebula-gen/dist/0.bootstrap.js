(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./nebula_gen.js":
/*!***********************!*\
  !*** ./nebula_gen.js ***!
  \***********************/
/*! exports provided: frame, __wbg_random_8cdd17579946bb97, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"frame\", function() { return frame; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_8cdd17579946bb97\", function() { return __wbg_random_8cdd17579946bb97; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./nebula_gen_bg */ \"./nebula_gen_bg.wasm\");\n/* tslint:disable */\n\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction passArray8ToWasm(arg) {\n    const ptr = _nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](arg.length * 1);\n    getUint8Memory().set(arg, ptr / 1);\n    return [ptr, arg.length];\n}\n/**\n* @param {Uint8Array} arg0\n* @param {boolean} arg1\n* @returns {void}\n*/\nfunction frame(arg0, arg1) {\n    const [ptr0, len0] = passArray8ToWasm(arg0);\n    try {\n        return _nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"frame\"](ptr0, len0, arg1 ? 1 : 0);\n\n    } finally {\n        arg0.set(getUint8Memory().subarray(ptr0 / 1, ptr0 / 1 + len0));\n        _nebula_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\nconst __wbg_random_8cdd17579946bb97_target = Math.random.bind(Math) || function() {\n    throw new Error(`wasm-bindgen: Math.random.bind(Math) does not exist`);\n};\n\nfunction __wbg_random_8cdd17579946bb97() {\n    return __wbg_random_8cdd17579946bb97_target();\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./nebula_gen.js?");

/***/ }),

/***/ "./nebula_gen_bg.wasm":
/*!****************************!*\
  !*** ./nebula_gen_bg.wasm ***!
  \****************************/
/*! exports provided: memory, __indirect_function_table, __heap_base, __data_end, frame, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./nebula_gen */ \"./nebula_gen.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./nebula_gen_bg.wasm?");

/***/ })

}]);