const fs = require('fs');
const path = require('path');

// Fix JavaScript file
const jsFile = 'target/site/pkg/koin.js';

if (fs.existsSync(jsFile)) {
    let content = fs.readFileSync(jsFile, 'utf8');

    // Fix deprecated warning
    content = content.replace(
        /console\.warn\('using deprecated parameters for the initialization function; pass a single object instead'\)/g,
        '// Deprecated warning suppressed for compatibility'
    );

    // Ensure fetch uses cors mode to match crossorigin="anonymous" preload
    content = content.replace(
        /fetch\(module_or_path\)(?!\,)/g,
        'fetch(module_or_path, { mode: "cors" })'
    );

    // Add comprehensive fix for WASM CORS issues
    const preloadFix = `
// Remove duplicate WASM preloads and ensure proper CORS mode
(function() {
    // Find all WASM preloads and handle duplicates
    const wasmPreloads = document.querySelectorAll('link[rel="preload"][href*=".wasm"]');
    let correctPreload = null;
    let incorrectPreloads = [];

    wasmPreloads.forEach(preload => {
        if (preload.hasAttribute("crossorigin")) {
            correctPreload = preload;
        } else {
            incorrectPreloads.push(preload);
        }
    });

    // Remove only the incorrect preloads (without crossorigin)
    incorrectPreloads.forEach(preload => {
        preload.remove();
    });

    // If no correct preload exists but we have incorrect ones, fix the first one
    if (!correctPreload && incorrectPreloads.length > 0) {
        // This shouldn't happen with our new approach, but just in case
        const wasmPreload = document.querySelector('link[rel="preload"][href*=".wasm"]');
        if (wasmPreload && !wasmPreload.hasAttribute("crossorigin")) {
            wasmPreload.setAttribute("crossorigin", "anonymous");
        }
    }

    // Fix JS modulepreload crossorigin attribute if needed
    const jsModulePreload = document.querySelector('link[rel="modulepreload"][href*="koin.js"]');
    if (jsModulePreload && !jsModulePreload.hasAttribute("crossorigin")) {
        jsModulePreload.setAttribute("crossorigin", "anonymous");
    }

    // Override fetch to ensure WASM requests use proper CORS mode
    if (typeof fetch !== 'undefined') {
        const originalFetch = fetch;
        window.fetch = function(input, init) {
            // Check if this is a WASM request
            const url = typeof input === 'string' ? input : input.url;
            if (url && url.includes('.wasm')) {
                // Ensure proper CORS mode for WASM requests
                const newInit = {
                    ...init,
                    mode: 'cors',
                    credentials: 'omit'
                };
                return originalFetch.call(this, input, newInit);
            }
            return originalFetch.call(this, input, init);
        };
    }
})();

`;

    content = preloadFix + content;

    fs.writeFileSync(jsFile, content);
}

// Also fix the generated HTML template if it exists
function fixServerGeneratedHtml() {
    // Check if there's a generated HTML template or server file that contains the preload
    const serverFile = 'target/release/koin';
    const serverFileWin = 'target/release/koin.exe';

    // Since we can't easily modify the compiled binary, let's try a different approach
    // We'll create a post-processing script that runs after the server generates HTML
}

// Enhanced HTML file fixing with better WASM preload detection
function fixHtmlFiles(dir) {
    if (!fs.existsSync(dir)) return;

    const files = fs.readdirSync(dir);
    files.forEach(file => {
        const filePath = path.join(dir, file);
        const stat = fs.statSync(filePath);

        if (stat.isDirectory()) {
            fixHtmlFiles(filePath);
        } else if (file.endsWith('.html') || file === 'index') {
            try {
                let content = fs.readFileSync(filePath, 'utf8');
                const originalContent = content;

                // Fix ALL WASM preload links to include crossorigin (more comprehensive)
                content = content.replace(
                    /<link([^>]*rel="preload"[^>]*href="[^"]*\.wasm"[^>]*)>/g,
                    (match, attrs) => {
                        if (!attrs.includes('crossorigin')) {
                            return match.replace('>', ' crossorigin="anonymous">');
                        }
                        return match;
                    }
                );

                // Fix specific koin.wasm preload
                content = content.replace(
                    /<link([^>]*rel="preload"[^>]*href="[^"]*koin\.wasm"[^>]*)>/g,
                    (match, attrs) => {
                        if (!attrs.includes('crossorigin')) {
                            return match.replace('>', ' crossorigin="anonymous">');
                        }
                        return match;
                    }
                );

                // Fix modulepreload links to include crossorigin
                content = content.replace(
                    /<link([^>]*rel="modulepreload"[^>]*href="[^"]*koin\.js"[^>]*)>/g,
                    (match, attrs) => {
                        if (!attrs.includes('crossorigin')) {
                            return match.replace('>', ' crossorigin="anonymous">');
                        }
                        return match;
                    }
                );

                if (content !== originalContent) {
                    fs.writeFileSync(filePath, content);
                }
            } catch (e) {
                // Ignore files that can't be processed
            }
        }
    });
}

// Check for HTML files in common locations
fixHtmlFiles('target/site');
fixHtmlFiles('target');

// Handle server-side HTML generation
fixServerGeneratedHtml();
