# README

This is a reproduction of an issue I've run into with creating and setting text on text objects inside of a wasm environment using pdfium-render. It's forked from the example in the pdfium-render library

## Running the example

* Make sure you have wasm-pack installed: `cargo install wasm-pack`
* Run the build script: `./build.sh`
  * It builds the wasm module via `wasm-pack build ./ --target no-modules`
  * It then copies over the two files to the release folder
  * Note that the two generated files are ignored by git 
* `cd release && ./serve.sh`

## File Structure

The release directory is mostly prebuilt and contains everything except the stuff we compile ourselves:
* index.html - this is a modified version of the index.html from the example. We only invoke the method for rendering the pdf, and have removed the metrics invocation
* test.pdf - a pdf I found off the internet that has multiple pages. Our code ends up downloading it and rendering it, but throws away the rendering before rending a problematic pdf that we've generated ourselves
* pdfium.js - the pdfium.js from the node folder in the https://github.com/paulocoutinhox/pdfium-lib/releases, release #6684
* pdfium.was - the pdfium.wasm from the same folder and release as the pdfium.js
* serve.sh - invoke this to serve the example on localhost:4000

