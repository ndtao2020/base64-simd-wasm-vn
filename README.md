# base64-simd-wasm-vn
[![npm](https://img.shields.io/npm/v/base64-simd-wasm-vn)](https://www.npmjs.com/package/base64-simd-wasm-vn)

## Browser

```js
import init, { encode, decode } from 'base64-simd-wasm-vn/web';

await init();

const data = "SIMD-accelerated base64 encoding and decoding"

const encoded = encode(new TextEncoder().encode(data));
const decoded = decode(encoded);

console.log("data: ", data);
console.log("encode: ", encoded);
console.log("decode: ", decoded);

const text = new TextDecoder().decode(decoded)

console.log("text: ", text);
```
