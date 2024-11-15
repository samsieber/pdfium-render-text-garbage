#!/usr/bin/env bash

wasm-pack build ./ --target no-modules && cp pkg/pdfium_render_text_garbage.js release/ && cp pkg/pdfium_render_text_garbage_bg.wasm release/