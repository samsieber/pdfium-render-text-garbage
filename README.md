# README

This is diagnosing an issue I've run into when rendering pdfs. They sometimes don't render correctly. 

## Running the example

* Make sure you have wasm-pack installed: `cargo install wasm-pack`
* Run the build script: `./build.sh`
  * It builds the wasm module via `wasm-pack build ./ --target no-modules`
  * It then copies over the two files to the release folder
  * Note that the two generated files are ignored by git 
* `cd release && ./serve.sh`

## Diagnosing the issue

Rendering a page more than once tends to improve the result. Displayed is a color pdf. If you go into index.html line 78 and switch to test.pdf you'll see the first page render noise, but then the subsequent renders work correctly.

## File Structure

The release directory is mostly prebuilt and contains everything except the stuff we compile ourselves:
* index.html - this is a modified version of the index.html from the example. We only invoke the method for rendering the pdf, and have removed the metrics invocation
* test.pdf - a pdf I found off the internet that has multiple pages. Our code ends up downloading it and rendering it, but throws away the rendering before rending a problematic pdf that we've generated ourselves
* pdfium.js - the pdfium.js from the node folder in the https://github.com/paulocoutinhox/pdfium-lib/releases, release #6684
* pdfium.was - the pdfium.wasm from the same folder and release as the pdfium.js
* serve.sh - invoke this to serve the example on localhost:4000

