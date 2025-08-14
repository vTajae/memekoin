var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import { connect as V } from "cloudflare:sockets";
import Tn from "./fe15f30bc4f3246dd8a07989006e9facb03d9c0e-index.wasm";
import { WorkerEntrypoint as kn } from "cloudflare:workers";
var C = Object.defineProperty;
var U = /* @__PURE__ */ __name((e, t) => {
  for (var n in t) C(e, n, { get: t[n], enumerable: true });
}, "U");
var w = {};
U(w, { IntoUnderlyingByteSource: /* @__PURE__ */ __name(() => k, "IntoUnderlyingByteSource"), IntoUnderlyingSink: /* @__PURE__ */ __name(() => I, "IntoUnderlyingSink"), IntoUnderlyingSource: /* @__PURE__ */ __name(() => x, "IntoUnderlyingSource"), MinifyConfig: /* @__PURE__ */ __name(() => y, "MinifyConfig"), PolishConfig: /* @__PURE__ */ __name(() => X, "PolishConfig"), R2Range: /* @__PURE__ */ __name(() => E, "R2Range"), RequestRedirect: /* @__PURE__ */ __name(() => J, "RequestRedirect"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(() => ee, "__wbg_String_8f0eb39a4a4c2f66"), __wbg_append_8c7dd8d641a5f01b: /* @__PURE__ */ __name(() => te, "__wbg_append_8c7dd8d641a5f01b"), __wbg_body_018617e858cb7195: /* @__PURE__ */ __name(() => ne, "__wbg_body_018617e858cb7195"), __wbg_body_0b8fd1fe671660df: /* @__PURE__ */ __name(() => re, "__wbg_body_0b8fd1fe671660df"), __wbg_buffer_09165b52af8c5237: /* @__PURE__ */ __name(() => _e, "__wbg_buffer_09165b52af8c5237"), __wbg_buffer_609cc3eee51ed158: /* @__PURE__ */ __name(() => oe, "__wbg_buffer_609cc3eee51ed158"), __wbg_byobRequest_77d9adf63337edfb: /* @__PURE__ */ __name(() => ce, "__wbg_byobRequest_77d9adf63337edfb"), __wbg_byteLength_e674b853d9c77e1d: /* @__PURE__ */ __name(() => ie, "__wbg_byteLength_e674b853d9c77e1d"), __wbg_byteOffset_fd862df290ef848d: /* @__PURE__ */ __name(() => se, "__wbg_byteOffset_fd862df290ef848d"), __wbg_call_672a4d21634d4a24: /* @__PURE__ */ __name(() => ue, "__wbg_call_672a4d21634d4a24"), __wbg_call_7cccdd69e0791ae2: /* @__PURE__ */ __name(() => fe, "__wbg_call_7cccdd69e0791ae2"), __wbg_cancel_8a308660caa6cadf: /* @__PURE__ */ __name(() => ae, "__wbg_cancel_8a308660caa6cadf"), __wbg_catch_a6e601879b2610e9: /* @__PURE__ */ __name(() => be, "__wbg_catch_a6e601879b2610e9"), __wbg_cause_9940c4e8dfcd5129: /* @__PURE__ */ __name(() => ge, "__wbg_cause_9940c4e8dfcd5129"), __wbg_cf_475e858e5c5db972: /* @__PURE__ */ __name(() => de, "__wbg_cf_475e858e5c5db972"), __wbg_cf_60aafe7bb03e919a: /* @__PURE__ */ __name(() => we, "__wbg_cf_60aafe7bb03e919a"), __wbg_close_24caca68e93b9c03: /* @__PURE__ */ __name(() => le, "__wbg_close_24caca68e93b9c03"), __wbg_close_304cc1fef3466669: /* @__PURE__ */ __name(() => pe, "__wbg_close_304cc1fef3466669"), __wbg_close_5ce03e29be453811: /* @__PURE__ */ __name(() => xe, "__wbg_close_5ce03e29be453811"), __wbg_connect_b4166aea43b7c262: /* @__PURE__ */ __name(() => ye, "__wbg_connect_b4166aea43b7c262"), __wbg_constructor_9fd96f589d65d4e5: /* @__PURE__ */ __name(() => he, "__wbg_constructor_9fd96f589d65d4e5"), __wbg_crypto_574e78ad8b13b65f: /* @__PURE__ */ __name(() => me, "__wbg_crypto_574e78ad8b13b65f"), __wbg_done_769e5ede4b31c67b: /* @__PURE__ */ __name(() => Re, "__wbg_done_769e5ede4b31c67b"), __wbg_enqueue_bb16ba72f537dc9e: /* @__PURE__ */ __name(() => Fe, "__wbg_enqueue_bb16ba72f537dc9e"), __wbg_entries_2a52db465d0421fb: /* @__PURE__ */ __name(() => Se, "__wbg_entries_2a52db465d0421fb"), __wbg_error_524f506f44df1645: /* @__PURE__ */ __name(() => Te, "__wbg_error_524f506f44df1645"), __wbg_error_7534b8e9a36f1ab4: /* @__PURE__ */ __name(() => ke, "__wbg_error_7534b8e9a36f1ab4"), __wbg_fetch_07cd86dd296a5a63: /* @__PURE__ */ __name(() => Ie, "__wbg_fetch_07cd86dd296a5a63"), __wbg_fetch_79398949f1862502: /* @__PURE__ */ __name(() => Ee, "__wbg_fetch_79398949f1862502"), __wbg_getRandomValues_38097e921c2494c3: /* @__PURE__ */ __name(() => Oe, "__wbg_getRandomValues_38097e921c2494c3"), __wbg_getRandomValues_3c9c0d586e575a16: /* @__PURE__ */ __name(() => je, "__wbg_getRandomValues_3c9c0d586e575a16"), __wbg_getRandomValues_b8f5dbd5f3995a9e: /* @__PURE__ */ __name(() => ze, "__wbg_getRandomValues_b8f5dbd5f3995a9e"), __wbg_getReader_48e00749fe3f6089: /* @__PURE__ */ __name(() => Le, "__wbg_getReader_48e00749fe3f6089"), __wbg_getReader_be0d36e5873a525b: /* @__PURE__ */ __name(() => qe, "__wbg_getReader_be0d36e5873a525b"), __wbg_getTime_46267b1c24877e30: /* @__PURE__ */ __name(() => Me, "__wbg_getTime_46267b1c24877e30"), __wbg_getWriter_6ce182d0adc3f96b: /* @__PURE__ */ __name(() => Ae, "__wbg_getWriter_6ce182d0adc3f96b"), __wbg_get_123509460060ab98: /* @__PURE__ */ __name(() => De, "__wbg_get_123509460060ab98"), __wbg_get_67b2ba62fc30de12: /* @__PURE__ */ __name(() => We, "__wbg_get_67b2ba62fc30de12"), __wbg_get_b9b93047fe3cf45b: /* @__PURE__ */ __name(() => Ce, "__wbg_get_b9b93047fe3cf45b"), __wbg_getdone_d47073731acd3e74: /* @__PURE__ */ __name(() => Ue, "__wbg_getdone_d47073731acd3e74"), __wbg_getvalue_009dcd63692bee1f: /* @__PURE__ */ __name(() => Ve, "__wbg_getvalue_009dcd63692bee1f"), __wbg_headers_7852a8ea641c1379: /* @__PURE__ */ __name(() => Pe, "__wbg_headers_7852a8ea641c1379"), __wbg_headers_9cb51cfd2ac780a4: /* @__PURE__ */ __name(() => $e, "__wbg_headers_9cb51cfd2ac780a4"), __wbg_httpProtocol_a32dd935f614e790: /* @__PURE__ */ __name(() => ve, "__wbg_httpProtocol_a32dd935f614e790"), __wbg_instanceof_Error_4d54113b22d20306: /* @__PURE__ */ __name(() => Ne, "__wbg_instanceof_Error_4d54113b22d20306"), __wbg_instanceof_ReadableStreamDefaultReader_056dcea99b3557aa: /* @__PURE__ */ __name(() => Be, "__wbg_instanceof_ReadableStreamDefaultReader_056dcea99b3557aa"), __wbg_instanceof_ReadableStream_87eac785b90f3611: /* @__PURE__ */ __name(() => He, "__wbg_instanceof_ReadableStream_87eac785b90f3611"), __wbg_instanceof_Response_f2cc20d9f7dfd644: /* @__PURE__ */ __name(() => Ge, "__wbg_instanceof_Response_f2cc20d9f7dfd644"), __wbg_length_a446193dc22c12f8: /* @__PURE__ */ __name(() => Xe, "__wbg_length_a446193dc22c12f8"), __wbg_log_c222819a41e063d3: /* @__PURE__ */ __name(() => Je, "__wbg_log_c222819a41e063d3"), __wbg_method_3dcc854b644c5a56: /* @__PURE__ */ __name(() => Ke, "__wbg_method_3dcc854b644c5a56"), __wbg_minifyconfig_new: /* @__PURE__ */ __name(() => Qe, "__wbg_minifyconfig_new"), __wbg_msCrypto_a61aeb35a24c1329: /* @__PURE__ */ __name(() => Ye, "__wbg_msCrypto_a61aeb35a24c1329"), __wbg_name_16617c8e9d4188ac: /* @__PURE__ */ __name(() => Ze, "__wbg_name_16617c8e9d4188ac"), __wbg_new0_f788a2397c7ca929: /* @__PURE__ */ __name(() => et, "__wbg_new0_f788a2397c7ca929"), __wbg_new_018dcc2d6c8c2f6a: /* @__PURE__ */ __name(() => tt, "__wbg_new_018dcc2d6c8c2f6a"), __wbg_new_23a2665fac83c611: /* @__PURE__ */ __name(() => nt, "__wbg_new_23a2665fac83c611"), __wbg_new_405e22f390576ce2: /* @__PURE__ */ __name(() => rt, "__wbg_new_405e22f390576ce2"), __wbg_new_5e0be73521bc8c17: /* @__PURE__ */ __name(() => _t, "__wbg_new_5e0be73521bc8c17"), __wbg_new_8a6f238a6ece86ea: /* @__PURE__ */ __name(() => ot, "__wbg_new_8a6f238a6ece86ea"), __wbg_new_a12002a7f91c75be: /* @__PURE__ */ __name(() => ct, "__wbg_new_a12002a7f91c75be"), __wbg_new_c68d7209be747379: /* @__PURE__ */ __name(() => it, "__wbg_new_c68d7209be747379"), __wbg_newnoargs_105ed471475aaf50: /* @__PURE__ */ __name(() => st, "__wbg_newnoargs_105ed471475aaf50"), __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a: /* @__PURE__ */ __name(() => ut, "__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a"), __wbg_newwithheaders_77fd1e80b866c52e: /* @__PURE__ */ __name(() => ft, "__wbg_newwithheaders_77fd1e80b866c52e"), __wbg_newwithintounderlyingsource_b47f6a6a596a7f24: /* @__PURE__ */ __name(() => at, "__wbg_newwithintounderlyingsource_b47f6a6a596a7f24"), __wbg_newwithlength_a381634e90c276d4: /* @__PURE__ */ __name(() => bt, "__wbg_newwithlength_a381634e90c276d4"), __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1: /* @__PURE__ */ __name(() => gt, "__wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1"), __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e: /* @__PURE__ */ __name(() => dt, "__wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e"), __wbg_newwithoptstrandinit_615a266ef226c260: /* @__PURE__ */ __name(() => wt, "__wbg_newwithoptstrandinit_615a266ef226c260"), __wbg_newwithstrandinit_06c535e0a867c635: /* @__PURE__ */ __name(() => lt, "__wbg_newwithstrandinit_06c535e0a867c635"), __wbg_next_6574e1a8a62d1055: /* @__PURE__ */ __name(() => pt, "__wbg_next_6574e1a8a62d1055"), __wbg_node_905d3e251edff8a2: /* @__PURE__ */ __name(() => xt, "__wbg_node_905d3e251edff8a2"), __wbg_process_dc0fbacc7c1c06f7: /* @__PURE__ */ __name(() => yt, "__wbg_process_dc0fbacc7c1c06f7"), __wbg_queueMicrotask_97d92b4fcc8a61c5: /* @__PURE__ */ __name(() => ht, "__wbg_queueMicrotask_97d92b4fcc8a61c5"), __wbg_queueMicrotask_d3219def82552485: /* @__PURE__ */ __name(() => mt, "__wbg_queueMicrotask_d3219def82552485"), __wbg_randomFillSync_ac0988aba3254290: /* @__PURE__ */ __name(() => Rt, "__wbg_randomFillSync_ac0988aba3254290"), __wbg_read_a2434af1186cb56c: /* @__PURE__ */ __name(() => Ft, "__wbg_read_a2434af1186cb56c"), __wbg_readable_e5665c153effc0ec: /* @__PURE__ */ __name(() => St, "__wbg_readable_e5665c153effc0ec"), __wbg_redirect_14b0c8193458f8c3: /* @__PURE__ */ __name(() => Tt, "__wbg_redirect_14b0c8193458f8c3"), __wbg_releaseLock_091899af97991d2e: /* @__PURE__ */ __name(() => kt, "__wbg_releaseLock_091899af97991d2e"), __wbg_releaseLock_a389e6ea62ce0f4d: /* @__PURE__ */ __name(() => It, "__wbg_releaseLock_a389e6ea62ce0f4d"), __wbg_require_60cc747a6bc5215a: /* @__PURE__ */ __name(() => Et, "__wbg_require_60cc747a6bc5215a"), __wbg_resolve_4851785c9c5f573d: /* @__PURE__ */ __name(() => Ot, "__wbg_resolve_4851785c9c5f573d"), __wbg_respond_1f279fa9f8edcb1c: /* @__PURE__ */ __name(() => jt, "__wbg_respond_1f279fa9f8edcb1c"), __wbg_set_11cd83f45504cedf: /* @__PURE__ */ __name(() => zt, "__wbg_set_11cd83f45504cedf"), __wbg_set_3f1d0b984ed272ed: /* @__PURE__ */ __name(() => Lt, "__wbg_set_3f1d0b984ed272ed"), __wbg_set_65595bdd868b3009: /* @__PURE__ */ __name(() => qt, "__wbg_set_65595bdd868b3009"), __wbg_set_8fc6bf8a5b1071d1: /* @__PURE__ */ __name(() => Mt, "__wbg_set_8fc6bf8a5b1071d1"), __wbg_set_bb8cecf6a62b9f46: /* @__PURE__ */ __name(() => At, "__wbg_set_bb8cecf6a62b9f46"), __wbg_set_wasm: /* @__PURE__ */ __name(() => O, "__wbg_set_wasm"), __wbg_setbody_5923b78a95eedf29: /* @__PURE__ */ __name(() => Dt, "__wbg_setbody_5923b78a95eedf29"), __wbg_setheaders_3b47c898e8de6d44: /* @__PURE__ */ __name(() => Wt, "__wbg_setheaders_3b47c898e8de6d44"), __wbg_setheaders_834c0bdb6a8949ad: /* @__PURE__ */ __name(() => Ct, "__wbg_setheaders_834c0bdb6a8949ad"), __wbg_sethighwatermark_793c99c89830c8e9: /* @__PURE__ */ __name(() => Ut, "__wbg_sethighwatermark_793c99c89830c8e9"), __wbg_setmethod_3c5280fe5d890842: /* @__PURE__ */ __name(() => Vt, "__wbg_setmethod_3c5280fe5d890842"), __wbg_setredirect_40e6a7f717a2f86a: /* @__PURE__ */ __name(() => Pt, "__wbg_setredirect_40e6a7f717a2f86a"), __wbg_setsignal_75b21ef3a81de905: /* @__PURE__ */ __name(() => $t, "__wbg_setsignal_75b21ef3a81de905"), __wbg_setstatus_51b4fc011091cbb3: /* @__PURE__ */ __name(() => vt, "__wbg_setstatus_51b4fc011091cbb3"), __wbg_signal_02f4435f82019061: /* @__PURE__ */ __name(() => Nt, "__wbg_signal_02f4435f82019061"), __wbg_stack_0ed75d68575b0f3c: /* @__PURE__ */ __name(() => Bt, "__wbg_stack_0ed75d68575b0f3c"), __wbg_startTls_be78d536b439568e: /* @__PURE__ */ __name(() => Ht, "__wbg_startTls_be78d536b439568e"), __wbg_static_accessor_GLOBAL_88a902d13a557d07: /* @__PURE__ */ __name(() => Gt, "__wbg_static_accessor_GLOBAL_88a902d13a557d07"), __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0: /* @__PURE__ */ __name(() => Xt, "__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0"), __wbg_static_accessor_SELF_37c5d418e4bf5819: /* @__PURE__ */ __name(() => Jt, "__wbg_static_accessor_SELF_37c5d418e4bf5819"), __wbg_static_accessor_WINDOW_5de37043a91a9c40: /* @__PURE__ */ __name(() => Kt, "__wbg_static_accessor_WINDOW_5de37043a91a9c40"), __wbg_status_f6360336ca686bf0: /* @__PURE__ */ __name(() => Qt, "__wbg_status_f6360336ca686bf0"), __wbg_subarray_aa9065fa9dc5df96: /* @__PURE__ */ __name(() => Yt, "__wbg_subarray_aa9065fa9dc5df96"), __wbg_then_44b73946d2fb3e7d: /* @__PURE__ */ __name(() => Zt, "__wbg_then_44b73946d2fb3e7d"), __wbg_then_48b406749878a531: /* @__PURE__ */ __name(() => en, "__wbg_then_48b406749878a531"), __wbg_toString_c813bbd34d063839: /* @__PURE__ */ __name(() => tn, "__wbg_toString_c813bbd34d063839"), __wbg_url_8f9653b899456042: /* @__PURE__ */ __name(() => nn, "__wbg_url_8f9653b899456042"), __wbg_value_cd1ffa7b1ab794f1: /* @__PURE__ */ __name(() => rn, "__wbg_value_cd1ffa7b1ab794f1"), __wbg_versions_c01dfd4722a88165: /* @__PURE__ */ __name(() => _n, "__wbg_versions_c01dfd4722a88165"), __wbg_view_fd8a56e8983f448d: /* @__PURE__ */ __name(() => on, "__wbg_view_fd8a56e8983f448d"), __wbg_webSocket_38528fcd2e5cba7f: /* @__PURE__ */ __name(() => cn, "__wbg_webSocket_38528fcd2e5cba7f"), __wbg_writable_7a0d4cd17b81163f: /* @__PURE__ */ __name(() => sn, "__wbg_writable_7a0d4cd17b81163f"), __wbg_write_311434e30ee214e5: /* @__PURE__ */ __name(() => un, "__wbg_write_311434e30ee214e5"), __wbindgen_cb_drop: /* @__PURE__ */ __name(() => fn, "__wbindgen_cb_drop"), __wbindgen_closure_wrapper4455: /* @__PURE__ */ __name(() => an, "__wbindgen_closure_wrapper4455"), __wbindgen_debug_string: /* @__PURE__ */ __name(() => bn, "__wbindgen_debug_string"), __wbindgen_error_new: /* @__PURE__ */ __name(() => gn, "__wbindgen_error_new"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(() => dn, "__wbindgen_init_externref_table"), __wbindgen_is_falsy: /* @__PURE__ */ __name(() => wn, "__wbindgen_is_falsy"), __wbindgen_is_function: /* @__PURE__ */ __name(() => ln, "__wbindgen_is_function"), __wbindgen_is_object: /* @__PURE__ */ __name(() => pn, "__wbindgen_is_object"), __wbindgen_is_string: /* @__PURE__ */ __name(() => xn, "__wbindgen_is_string"), __wbindgen_is_undefined: /* @__PURE__ */ __name(() => yn, "__wbindgen_is_undefined"), __wbindgen_memory: /* @__PURE__ */ __name(() => hn, "__wbindgen_memory"), __wbindgen_number_new: /* @__PURE__ */ __name(() => mn, "__wbindgen_number_new"), __wbindgen_string_get: /* @__PURE__ */ __name(() => Rn, "__wbindgen_string_get"), __wbindgen_string_new: /* @__PURE__ */ __name(() => Fn, "__wbindgen_string_new"), __wbindgen_throw: /* @__PURE__ */ __name(() => Sn, "__wbindgen_throw"), fetch: /* @__PURE__ */ __name(() => j, "fetch"), start: /* @__PURE__ */ __name(() => B, "start") });
var r;
function O(e) {
  r = e;
}
__name(O, "O");
var d = 0;
var R = null;
function h() {
  return (R === null || R.byteLength === 0) && (R = new Uint8Array(r.memory.buffer)), R;
}
__name(h, "h");
var P = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var F = new P("utf-8");
var $ = typeof F.encodeInto == "function" ? function(e, t) {
  return F.encodeInto(e, t);
} : function(e, t) {
  let n = F.encode(e);
  return t.set(n), { read: e.length, written: n.length };
};
function l(e, t, n) {
  if (n === void 0) {
    let f = F.encode(e), m = t(f.length, 1) >>> 0;
    return h().subarray(m, m + f.length).set(f), d = f.length, m;
  }
  let _ = e.length, o = t(_, 1) >>> 0, u = h(), i = 0;
  for (; i < _; i++) {
    let f = e.charCodeAt(i);
    if (f > 127) break;
    u[o + i] = f;
  }
  if (i !== _) {
    i !== 0 && (e = e.slice(i)), o = n(o, _, _ = i + e.length * 3, 1) >>> 0;
    let f = h().subarray(o + i, o + _);
    i += $(e, f).written, o = n(o, _, i, 1) >>> 0;
  }
  return d = i, o;
}
__name(l, "l");
var p = null;
function a() {
  return (p === null || p.buffer.detached === true || p.buffer.detached === void 0 && p.buffer !== r.memory.buffer) && (p = new DataView(r.memory.buffer)), p;
}
__name(a, "a");
var v = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var M = new v("utf-8", { ignoreBOM: true, fatal: true });
M.decode();
function b(e, t) {
  return e = e >>> 0, M.decode(h().subarray(e, e + t));
}
__name(b, "b");
function g(e) {
  let t = r.__externref_table_alloc();
  return r.__wbindgen_export_4.set(t, e), t;
}
__name(g, "g");
function c(e, t) {
  try {
    return e.apply(this, t);
  } catch (n) {
    let _ = g(n);
    r.__wbindgen_exn_store(_);
  }
}
__name(c, "c");
function s(e) {
  return e == null;
}
__name(s, "s");
function A(e, t) {
  return e = e >>> 0, h().subarray(e / 1, e / 1 + t);
}
__name(A, "A");
var z = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => {
  r.__wbindgen_export_6.get(e.dtor)(e.a, e.b);
});
function N(e, t, n, _) {
  let o = { a: e, b: t, cnt: 1, dtor: n }, u = /* @__PURE__ */ __name((...i) => {
    o.cnt++;
    let f = o.a;
    o.a = 0;
    try {
      return _(f, o.b, ...i);
    } finally {
      --o.cnt === 0 ? (r.__wbindgen_export_6.get(o.dtor)(f, o.b), z.unregister(o)) : o.a = f;
    }
  }, "u");
  return u.original = o, z.register(u, o, o), u;
}
__name(N, "N");
function T(e) {
  let t = typeof e;
  if (t == "number" || t == "boolean" || e == null) return `${e}`;
  if (t == "string") return `"${e}"`;
  if (t == "symbol") {
    let o = e.description;
    return o == null ? "Symbol" : `Symbol(${o})`;
  }
  if (t == "function") {
    let o = e.name;
    return typeof o == "string" && o.length > 0 ? `Function(${o})` : "Function";
  }
  if (Array.isArray(e)) {
    let o = e.length, u = "[";
    o > 0 && (u += T(e[0]));
    for (let i = 1; i < o; i++) u += ", " + T(e[i]);
    return u += "]", u;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(e)), _;
  if (n && n.length > 1) _ = n[1];
  else return toString.call(e);
  if (_ == "Object") try {
    return "Object(" + JSON.stringify(e) + ")";
  } catch {
    return "Object";
  }
  return e instanceof Error ? `${e.name}: ${e.message}
${e.stack}` : _;
}
__name(T, "T");
function B() {
  r.start();
}
__name(B, "B");
function j(e, t, n) {
  return r.fetch(e, t, n);
}
__name(j, "j");
function H(e, t, n) {
  r.closure1660_externref_shim(e, t, n);
}
__name(H, "H");
function G(e, t, n, _) {
  r.closure1702_externref_shim(e, t, n, _);
}
__name(G, "G");
var X = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var J = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var K = ["bytes"];
var D = ["follow", "error", "manual"];
var Q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => r.__wbg_intounderlyingbytesource_free(e >>> 0, 1));
var k = class {
  static {
    __name(this, "k");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Q.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingbytesource_free(t, 0);
  }
  get type() {
    let t = r.intounderlyingbytesource_type(this.__wbg_ptr);
    return K[t];
  }
  get autoAllocateChunkSize() {
    return r.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(t) {
    r.intounderlyingbytesource_start(this.__wbg_ptr, t);
  }
  pull(t) {
    return r.intounderlyingbytesource_pull(this.__wbg_ptr, t);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingbytesource_cancel(t);
  }
};
var Y = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => r.__wbg_intounderlyingsink_free(e >>> 0, 1));
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Y.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsink_free(t, 0);
  }
  write(t) {
    return r.intounderlyingsink_write(this.__wbg_ptr, t);
  }
  close() {
    let t = this.__destroy_into_raw();
    return r.intounderlyingsink_close(t);
  }
  abort(t) {
    let n = this.__destroy_into_raw();
    return r.intounderlyingsink_abort(n, t);
  }
};
var L = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => r.__wbg_intounderlyingsource_free(e >>> 0, 1));
var x = class {
  static {
    __name(this, "x");
  }
  static __wrap(t) {
    t = t >>> 0;
    let n = Object.create(x.prototype);
    return n.__wbg_ptr = t, L.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, L.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsource_free(t, 0);
  }
  pull(t) {
    return r.intounderlyingsource_pull(this.__wbg_ptr, t);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingsource_cancel(t);
  }
};
var q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => r.__wbg_minifyconfig_free(e >>> 0, 1));
var y = class {
  static {
    __name(this, "y");
  }
  static __wrap(t) {
    t = t >>> 0;
    let n = Object.create(y.prototype);
    return n.__wbg_ptr = t, q.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, q.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_minifyconfig_free(t, 0);
  }
  get js() {
    return r.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(t) {
    r.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
  get html() {
    return r.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(t) {
    r.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  get css() {
    return r.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    r.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
};
var Z = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((e) => r.__wbg_r2range_free(e >>> 0, 1));
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Z.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_r2range_free(t, 0);
  }
  get offset() {
    let t = r.__wbg_get_r2range_offset(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set offset(t) {
    r.__wbg_set_r2range_offset(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
  get length() {
    let t = r.__wbg_get_r2range_length(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set length(t) {
    r.__wbg_set_r2range_length(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
  get suffix() {
    let t = r.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set suffix(t) {
    r.__wbg_set_r2range_suffix(this.__wbg_ptr, !s(t), s(t) ? 0 : t);
  }
};
function ee(e, t) {
  let n = String(t), _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
  a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
}
__name(ee, "ee");
function te() {
  return c(function(e, t, n, _, o) {
    e.append(b(t, n), b(_, o));
  }, arguments);
}
__name(te, "te");
function ne(e) {
  let t = e.body;
  return s(t) ? 0 : g(t);
}
__name(ne, "ne");
function re(e) {
  let t = e.body;
  return s(t) ? 0 : g(t);
}
__name(re, "re");
function _e(e) {
  return e.buffer;
}
__name(_e, "_e");
function oe(e) {
  return e.buffer;
}
__name(oe, "oe");
function ce(e) {
  let t = e.byobRequest;
  return s(t) ? 0 : g(t);
}
__name(ce, "ce");
function ie(e) {
  return e.byteLength;
}
__name(ie, "ie");
function se(e) {
  return e.byteOffset;
}
__name(se, "se");
function ue() {
  return c(function(e, t) {
    return e.call(t);
  }, arguments);
}
__name(ue, "ue");
function fe() {
  return c(function(e, t, n) {
    return e.call(t, n);
  }, arguments);
}
__name(fe, "fe");
function ae(e) {
  return e.cancel();
}
__name(ae, "ae");
function be(e, t) {
  return e.catch(t);
}
__name(be, "be");
function ge(e) {
  return e.cause;
}
__name(ge, "ge");
function de() {
  return c(function(e) {
    let t = e.cf;
    return s(t) ? 0 : g(t);
  }, arguments);
}
__name(de, "de");
function we() {
  return c(function(e) {
    let t = e.cf;
    return s(t) ? 0 : g(t);
  }, arguments);
}
__name(we, "we");
function le(e) {
  return e.close();
}
__name(le, "le");
function pe() {
  return c(function(e) {
    e.close();
  }, arguments);
}
__name(pe, "pe");
function xe() {
  return c(function(e) {
    e.close();
  }, arguments);
}
__name(xe, "xe");
function ye() {
  return c(function(e, t) {
    return V(e, t);
  }, arguments);
}
__name(ye, "ye");
function he(e) {
  return e.constructor;
}
__name(he, "he");
function me(e) {
  return e.crypto;
}
__name(me, "me");
function Re(e) {
  return e.done;
}
__name(Re, "Re");
function Fe() {
  return c(function(e, t) {
    e.enqueue(t);
  }, arguments);
}
__name(Fe, "Fe");
function Se(e) {
  return e.entries();
}
__name(Se, "Se");
function Te(e) {
  console.error(e);
}
__name(Te, "Te");
function ke(e, t) {
  let n, _;
  try {
    n = e, _ = t, console.error(b(e, t));
  } finally {
    r.__wbindgen_free(n, _, 1);
  }
}
__name(ke, "ke");
function Ie(e, t, n) {
  return e.fetch(t, n);
}
__name(Ie, "Ie");
function Ee(e, t, n, _) {
  return e.fetch(b(t, n), _);
}
__name(Ee, "Ee");
function Oe() {
  return c(function(e, t) {
    globalThis.crypto.getRandomValues(A(e, t));
  }, arguments);
}
__name(Oe, "Oe");
function je() {
  return c(function(e, t) {
    globalThis.crypto.getRandomValues(A(e, t));
  }, arguments);
}
__name(je, "je");
function ze() {
  return c(function(e, t) {
    e.getRandomValues(t);
  }, arguments);
}
__name(ze, "ze");
function Le() {
  return c(function(e) {
    return e.getReader();
  }, arguments);
}
__name(Le, "Le");
function qe(e) {
  return e.getReader();
}
__name(qe, "qe");
function Me(e) {
  return e.getTime();
}
__name(Me, "Me");
function Ae() {
  return c(function(e) {
    return e.getWriter();
  }, arguments);
}
__name(Ae, "Ae");
function De() {
  return c(function(e, t, n, _) {
    let o = t.get(b(n, _));
    var u = s(o) ? 0 : l(o, r.__wbindgen_malloc, r.__wbindgen_realloc), i = d;
    a().setInt32(e + 4 * 1, i, true), a().setInt32(e + 4 * 0, u, true);
  }, arguments);
}
__name(De, "De");
function We() {
  return c(function(e, t) {
    return Reflect.get(e, t);
  }, arguments);
}
__name(We, "We");
function Ce(e, t) {
  return e[t >>> 0];
}
__name(Ce, "Ce");
function Ue(e) {
  let t = e.done;
  return s(t) ? 16777215 : t ? 1 : 0;
}
__name(Ue, "Ue");
function Ve(e) {
  return e.value;
}
__name(Ve, "Ve");
function Pe(e) {
  return e.headers;
}
__name(Pe, "Pe");
function $e(e) {
  return e.headers;
}
__name($e, "$e");
function ve() {
  return c(function(e, t) {
    let n = t.httpProtocol, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
    a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
  }, arguments);
}
__name(ve, "ve");
function Ne(e) {
  let t;
  try {
    t = e instanceof Error;
  } catch {
    t = false;
  }
  return t;
}
__name(Ne, "Ne");
function Be(e) {
  let t;
  try {
    t = e instanceof ReadableStreamDefaultReader;
  } catch {
    t = false;
  }
  return t;
}
__name(Be, "Be");
function He(e) {
  let t;
  try {
    t = e instanceof ReadableStream;
  } catch {
    t = false;
  }
  return t;
}
__name(He, "He");
function Ge(e) {
  let t;
  try {
    t = e instanceof Response;
  } catch {
    t = false;
  }
  return t;
}
__name(Ge, "Ge");
function Xe(e) {
  return e.length;
}
__name(Xe, "Xe");
function Je(e) {
  console.log(e);
}
__name(Je, "Je");
function Ke(e, t) {
  let n = t.method, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
  a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
}
__name(Ke, "Ke");
function Qe(e) {
  return y.__wrap(e);
}
__name(Qe, "Qe");
function Ye(e) {
  return e.msCrypto;
}
__name(Ye, "Ye");
function Ze(e) {
  return e.name;
}
__name(Ze, "Ze");
function et() {
  return /* @__PURE__ */ new Date();
}
__name(et, "et");
function tt() {
  return c(function() {
    return new Headers();
  }, arguments);
}
__name(tt, "tt");
function nt(e, t) {
  try {
    var n = { a: e, b: t }, _ = /* @__PURE__ */ __name((u, i) => {
      let f = n.a;
      n.a = 0;
      try {
        return G(f, n.b, u, i);
      } finally {
        n.a = f;
      }
    }, "_");
    return new Promise(_);
  } finally {
    n.a = n.b = 0;
  }
}
__name(nt, "nt");
function rt() {
  return new Object();
}
__name(rt, "rt");
function _t() {
  return /* @__PURE__ */ new Map();
}
__name(_t, "_t");
function ot() {
  return new Error();
}
__name(ot, "ot");
function ct(e) {
  return new Uint8Array(e);
}
__name(ct, "ct");
function it(e, t) {
  return new Error(b(e, t));
}
__name(it, "it");
function st(e, t) {
  return new Function(b(e, t));
}
__name(st, "st");
function ut(e, t, n) {
  return new Uint8Array(e, t >>> 0, n >>> 0);
}
__name(ut, "ut");
function ft() {
  return c(function(e) {
    return new Headers(e);
  }, arguments);
}
__name(ft, "ft");
function at(e, t) {
  return new ReadableStream(x.__wrap(e), t);
}
__name(at, "at");
function bt(e) {
  return new Uint8Array(e >>> 0);
}
__name(bt, "bt");
function gt() {
  return c(function(e, t) {
    return new Response(e, t);
  }, arguments);
}
__name(gt, "gt");
function dt() {
  return c(function(e, t) {
    return new Response(e, t);
  }, arguments);
}
__name(dt, "dt");
function wt() {
  return c(function(e, t, n) {
    return new Response(e === 0 ? void 0 : b(e, t), n);
  }, arguments);
}
__name(wt, "wt");
function lt() {
  return c(function(e, t, n) {
    return new Request(b(e, t), n);
  }, arguments);
}
__name(lt, "lt");
function pt() {
  return c(function(e) {
    return e.next();
  }, arguments);
}
__name(pt, "pt");
function xt(e) {
  return e.node;
}
__name(xt, "xt");
function yt(e) {
  return e.process;
}
__name(yt, "yt");
function ht(e) {
  queueMicrotask(e);
}
__name(ht, "ht");
function mt(e) {
  return e.queueMicrotask;
}
__name(mt, "mt");
function Rt() {
  return c(function(e, t) {
    e.randomFillSync(t);
  }, arguments);
}
__name(Rt, "Rt");
function Ft(e) {
  return e.read();
}
__name(Ft, "Ft");
function St() {
  return c(function(e) {
    return e.readable;
  }, arguments);
}
__name(St, "St");
function Tt(e) {
  let t = e.redirect;
  return (D.indexOf(t) + 1 || 4) - 1;
}
__name(Tt, "Tt");
function kt(e) {
  e.releaseLock();
}
__name(kt, "kt");
function It(e) {
  e.releaseLock();
}
__name(It, "It");
function Et() {
  return c(function() {
    return module.require;
  }, arguments);
}
__name(Et, "Et");
function Ot(e) {
  return Promise.resolve(e);
}
__name(Ot, "Ot");
function jt() {
  return c(function(e, t) {
    e.respond(t >>> 0);
  }, arguments);
}
__name(jt, "jt");
function zt() {
  return c(function(e, t, n, _, o) {
    e.set(b(t, n), b(_, o));
  }, arguments);
}
__name(zt, "zt");
function Lt(e, t, n) {
  e[t] = n;
}
__name(Lt, "Lt");
function qt(e, t, n) {
  e.set(t, n >>> 0);
}
__name(qt, "qt");
function Mt(e, t, n) {
  return e.set(t, n);
}
__name(Mt, "Mt");
function At() {
  return c(function(e, t, n) {
    return Reflect.set(e, t, n);
  }, arguments);
}
__name(At, "At");
function Dt(e, t) {
  e.body = t;
}
__name(Dt, "Dt");
function Wt(e, t) {
  e.headers = t;
}
__name(Wt, "Wt");
function Ct(e, t) {
  e.headers = t;
}
__name(Ct, "Ct");
function Ut(e, t) {
  e.highWaterMark = t;
}
__name(Ut, "Ut");
function Vt(e, t, n) {
  e.method = b(t, n);
}
__name(Vt, "Vt");
function Pt(e, t) {
  e.redirect = D[t];
}
__name(Pt, "Pt");
function $t(e, t) {
  e.signal = t;
}
__name($t, "$t");
function vt(e, t) {
  e.status = t;
}
__name(vt, "vt");
function Nt(e) {
  return e.signal;
}
__name(Nt, "Nt");
function Bt(e, t) {
  let n = t.stack, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
  a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
}
__name(Bt, "Bt");
function Ht() {
  return c(function(e) {
    return e.startTls();
  }, arguments);
}
__name(Ht, "Ht");
function Gt() {
  let e = typeof global > "u" ? null : global;
  return s(e) ? 0 : g(e);
}
__name(Gt, "Gt");
function Xt() {
  let e = typeof globalThis > "u" ? null : globalThis;
  return s(e) ? 0 : g(e);
}
__name(Xt, "Xt");
function Jt() {
  let e = typeof self > "u" ? null : self;
  return s(e) ? 0 : g(e);
}
__name(Jt, "Jt");
function Kt() {
  let e = typeof window > "u" ? null : window;
  return s(e) ? 0 : g(e);
}
__name(Kt, "Kt");
function Qt(e) {
  return e.status;
}
__name(Qt, "Qt");
function Yt(e, t, n) {
  return e.subarray(t >>> 0, n >>> 0);
}
__name(Yt, "Yt");
function Zt(e, t) {
  return e.then(t);
}
__name(Zt, "Zt");
function en(e, t, n) {
  return e.then(t, n);
}
__name(en, "en");
function tn(e) {
  return e.toString();
}
__name(tn, "tn");
function nn(e, t) {
  let n = t.url, _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
  a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
}
__name(nn, "nn");
function rn(e) {
  return e.value;
}
__name(rn, "rn");
function _n(e) {
  return e.versions;
}
__name(_n, "_n");
function on(e) {
  let t = e.view;
  return s(t) ? 0 : g(t);
}
__name(on, "on");
function cn() {
  return c(function(e) {
    let t = e.webSocket;
    return s(t) ? 0 : g(t);
  }, arguments);
}
__name(cn, "cn");
function sn() {
  return c(function(e) {
    return e.writable;
  }, arguments);
}
__name(sn, "sn");
function un(e, t) {
  return e.write(t);
}
__name(un, "un");
function fn(e) {
  let t = e.original;
  return t.cnt-- == 1 ? (t.a = 0, true) : false;
}
__name(fn, "fn");
function an(e, t, n) {
  return N(e, t, 1661, H);
}
__name(an, "an");
function bn(e, t) {
  let n = T(t), _ = l(n, r.__wbindgen_malloc, r.__wbindgen_realloc), o = d;
  a().setInt32(e + 4 * 1, o, true), a().setInt32(e + 4 * 0, _, true);
}
__name(bn, "bn");
function gn(e, t) {
  return new Error(b(e, t));
}
__name(gn, "gn");
function dn() {
  let e = r.__wbindgen_export_4, t = e.grow(4);
  e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
}
__name(dn, "dn");
function wn(e) {
  return !e;
}
__name(wn, "wn");
function ln(e) {
  return typeof e == "function";
}
__name(ln, "ln");
function pn(e) {
  let t = e;
  return typeof t == "object" && t !== null;
}
__name(pn, "pn");
function xn(e) {
  return typeof e == "string";
}
__name(xn, "xn");
function yn(e) {
  return e === void 0;
}
__name(yn, "yn");
function hn() {
  return r.memory;
}
__name(hn, "hn");
function mn(e) {
  return e;
}
__name(mn, "mn");
function Rn(e, t) {
  let n = t, _ = typeof n == "string" ? n : void 0;
  var o = s(_) ? 0 : l(_, r.__wbindgen_malloc, r.__wbindgen_realloc), u = d;
  a().setInt32(e + 4 * 1, u, true), a().setInt32(e + 4 * 0, o, true);
}
__name(Rn, "Rn");
function Fn(e, t) {
  return b(e, t);
}
__name(Fn, "Fn");
function Sn(e, t) {
  throw new Error(b(e, t));
}
__name(Sn, "Sn");
var W = new WebAssembly.Instance(Tn, { "./index_bg.js": w });
O(W.exports);
W.exports.__wbindgen_start?.();
var S = class extends kn {
  static {
    __name(this, "S");
  }
  async fetch(t) {
    return await j(t, this.env, this.ctx);
  }
  async queue(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
  async scheduled(t) {
    return await (void 0)(t, this.env, this.ctx);
  }
};
var In = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(w).map((e) => {
  In.includes(e) | e.startsWith("__") || (S.prototype[e] = w[e]);
});
var Ln = S;
export {
  k as IntoUnderlyingByteSource,
  I as IntoUnderlyingSink,
  x as IntoUnderlyingSource,
  y as MinifyConfig,
  X as PolishConfig,
  E as R2Range,
  J as RequestRedirect,
  ee as __wbg_String_8f0eb39a4a4c2f66,
  te as __wbg_append_8c7dd8d641a5f01b,
  ne as __wbg_body_018617e858cb7195,
  re as __wbg_body_0b8fd1fe671660df,
  _e as __wbg_buffer_09165b52af8c5237,
  oe as __wbg_buffer_609cc3eee51ed158,
  ce as __wbg_byobRequest_77d9adf63337edfb,
  ie as __wbg_byteLength_e674b853d9c77e1d,
  se as __wbg_byteOffset_fd862df290ef848d,
  ue as __wbg_call_672a4d21634d4a24,
  fe as __wbg_call_7cccdd69e0791ae2,
  ae as __wbg_cancel_8a308660caa6cadf,
  be as __wbg_catch_a6e601879b2610e9,
  ge as __wbg_cause_9940c4e8dfcd5129,
  de as __wbg_cf_475e858e5c5db972,
  we as __wbg_cf_60aafe7bb03e919a,
  le as __wbg_close_24caca68e93b9c03,
  pe as __wbg_close_304cc1fef3466669,
  xe as __wbg_close_5ce03e29be453811,
  ye as __wbg_connect_b4166aea43b7c262,
  he as __wbg_constructor_9fd96f589d65d4e5,
  me as __wbg_crypto_574e78ad8b13b65f,
  Re as __wbg_done_769e5ede4b31c67b,
  Fe as __wbg_enqueue_bb16ba72f537dc9e,
  Se as __wbg_entries_2a52db465d0421fb,
  Te as __wbg_error_524f506f44df1645,
  ke as __wbg_error_7534b8e9a36f1ab4,
  Ie as __wbg_fetch_07cd86dd296a5a63,
  Ee as __wbg_fetch_79398949f1862502,
  Oe as __wbg_getRandomValues_38097e921c2494c3,
  je as __wbg_getRandomValues_3c9c0d586e575a16,
  ze as __wbg_getRandomValues_b8f5dbd5f3995a9e,
  Le as __wbg_getReader_48e00749fe3f6089,
  qe as __wbg_getReader_be0d36e5873a525b,
  Me as __wbg_getTime_46267b1c24877e30,
  Ae as __wbg_getWriter_6ce182d0adc3f96b,
  De as __wbg_get_123509460060ab98,
  We as __wbg_get_67b2ba62fc30de12,
  Ce as __wbg_get_b9b93047fe3cf45b,
  Ue as __wbg_getdone_d47073731acd3e74,
  Ve as __wbg_getvalue_009dcd63692bee1f,
  Pe as __wbg_headers_7852a8ea641c1379,
  $e as __wbg_headers_9cb51cfd2ac780a4,
  ve as __wbg_httpProtocol_a32dd935f614e790,
  Ne as __wbg_instanceof_Error_4d54113b22d20306,
  Be as __wbg_instanceof_ReadableStreamDefaultReader_056dcea99b3557aa,
  He as __wbg_instanceof_ReadableStream_87eac785b90f3611,
  Ge as __wbg_instanceof_Response_f2cc20d9f7dfd644,
  Xe as __wbg_length_a446193dc22c12f8,
  Je as __wbg_log_c222819a41e063d3,
  Ke as __wbg_method_3dcc854b644c5a56,
  Qe as __wbg_minifyconfig_new,
  Ye as __wbg_msCrypto_a61aeb35a24c1329,
  Ze as __wbg_name_16617c8e9d4188ac,
  et as __wbg_new0_f788a2397c7ca929,
  tt as __wbg_new_018dcc2d6c8c2f6a,
  nt as __wbg_new_23a2665fac83c611,
  rt as __wbg_new_405e22f390576ce2,
  _t as __wbg_new_5e0be73521bc8c17,
  ot as __wbg_new_8a6f238a6ece86ea,
  ct as __wbg_new_a12002a7f91c75be,
  it as __wbg_new_c68d7209be747379,
  st as __wbg_newnoargs_105ed471475aaf50,
  ut as __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a,
  ft as __wbg_newwithheaders_77fd1e80b866c52e,
  at as __wbg_newwithintounderlyingsource_b47f6a6a596a7f24,
  bt as __wbg_newwithlength_a381634e90c276d4,
  gt as __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1,
  dt as __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e,
  wt as __wbg_newwithoptstrandinit_615a266ef226c260,
  lt as __wbg_newwithstrandinit_06c535e0a867c635,
  pt as __wbg_next_6574e1a8a62d1055,
  xt as __wbg_node_905d3e251edff8a2,
  yt as __wbg_process_dc0fbacc7c1c06f7,
  ht as __wbg_queueMicrotask_97d92b4fcc8a61c5,
  mt as __wbg_queueMicrotask_d3219def82552485,
  Rt as __wbg_randomFillSync_ac0988aba3254290,
  Ft as __wbg_read_a2434af1186cb56c,
  St as __wbg_readable_e5665c153effc0ec,
  Tt as __wbg_redirect_14b0c8193458f8c3,
  kt as __wbg_releaseLock_091899af97991d2e,
  It as __wbg_releaseLock_a389e6ea62ce0f4d,
  Et as __wbg_require_60cc747a6bc5215a,
  Ot as __wbg_resolve_4851785c9c5f573d,
  jt as __wbg_respond_1f279fa9f8edcb1c,
  zt as __wbg_set_11cd83f45504cedf,
  Lt as __wbg_set_3f1d0b984ed272ed,
  qt as __wbg_set_65595bdd868b3009,
  Mt as __wbg_set_8fc6bf8a5b1071d1,
  At as __wbg_set_bb8cecf6a62b9f46,
  O as __wbg_set_wasm,
  Dt as __wbg_setbody_5923b78a95eedf29,
  Wt as __wbg_setheaders_3b47c898e8de6d44,
  Ct as __wbg_setheaders_834c0bdb6a8949ad,
  Ut as __wbg_sethighwatermark_793c99c89830c8e9,
  Vt as __wbg_setmethod_3c5280fe5d890842,
  Pt as __wbg_setredirect_40e6a7f717a2f86a,
  $t as __wbg_setsignal_75b21ef3a81de905,
  vt as __wbg_setstatus_51b4fc011091cbb3,
  Nt as __wbg_signal_02f4435f82019061,
  Bt as __wbg_stack_0ed75d68575b0f3c,
  Ht as __wbg_startTls_be78d536b439568e,
  Gt as __wbg_static_accessor_GLOBAL_88a902d13a557d07,
  Xt as __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0,
  Jt as __wbg_static_accessor_SELF_37c5d418e4bf5819,
  Kt as __wbg_static_accessor_WINDOW_5de37043a91a9c40,
  Qt as __wbg_status_f6360336ca686bf0,
  Yt as __wbg_subarray_aa9065fa9dc5df96,
  Zt as __wbg_then_44b73946d2fb3e7d,
  en as __wbg_then_48b406749878a531,
  tn as __wbg_toString_c813bbd34d063839,
  nn as __wbg_url_8f9653b899456042,
  rn as __wbg_value_cd1ffa7b1ab794f1,
  _n as __wbg_versions_c01dfd4722a88165,
  on as __wbg_view_fd8a56e8983f448d,
  cn as __wbg_webSocket_38528fcd2e5cba7f,
  sn as __wbg_writable_7a0d4cd17b81163f,
  un as __wbg_write_311434e30ee214e5,
  fn as __wbindgen_cb_drop,
  an as __wbindgen_closure_wrapper4455,
  bn as __wbindgen_debug_string,
  gn as __wbindgen_error_new,
  dn as __wbindgen_init_externref_table,
  wn as __wbindgen_is_falsy,
  ln as __wbindgen_is_function,
  pn as __wbindgen_is_object,
  xn as __wbindgen_is_string,
  yn as __wbindgen_is_undefined,
  hn as __wbindgen_memory,
  mn as __wbindgen_number_new,
  Rn as __wbindgen_string_get,
  Fn as __wbindgen_string_new,
  Sn as __wbindgen_throw,
  Ln as default,
  j as fetch,
  B as start,
  Tn as wasmModule
};
//# sourceMappingURL=shim.js.map
