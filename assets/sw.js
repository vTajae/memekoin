// Enhanced Service Worker for Meme Koin App - Performance Optimized
const CACHE_NAME = 'meme-koin-v2';
const RUNTIME_CACHE = 'meme-koin-runtime-v2';

// Critical assets that should be cached immediately
const CRITICAL_ASSETS = [
  '/pkg/koin.css',
  '/pkg/koin.js',
  '/pkg/koin_bg.wasm',
  '/',
];

// Assets that can be cached on demand
const RUNTIME_ASSETS = [
  '/favicon.ico',
  '/manifest.json',
];

// Install - cache critical assets with retry logic
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(async (cache) => {
        // Cache critical assets with retry logic
        const cachePromises = CRITICAL_ASSETS.map(async (url) => {
          try {
            const response = await fetch(url);
            if (response.ok) {
              return cache.put(url, response);
            }
          } catch (error) {
            console.warn(`Failed to cache ${url}:`, error);
          }
        });

        await Promise.allSettled(cachePromises);
        return self.skipWaiting();
      })
  );
});

// Activate - clean old caches and claim clients
self.addEventListener('activate', (event) => {
  event.waitUntil(
    Promise.all([
      // Clean old caches
      caches.keys().then((cacheNames) =>
        Promise.all(
          cacheNames.map((cacheName) =>
            cacheName !== CACHE_NAME && cacheName !== RUNTIME_CACHE
              ? caches.delete(cacheName)
              : null
          )
        )
      ),
      // Claim all clients
      self.clients.claim()
    ])
  );
});

// Fetch - optimized caching strategy
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Handle different types of requests with appropriate strategies
  if (url.pathname.startsWith('/pkg/')) {
    // Static assets: Cache first with long-term caching
    event.respondWith(cacheFirst(request));
  } else if (url.pathname === '/' || url.pathname.endsWith('.html')) {
    // HTML pages: Network first with cache fallback
    event.respondWith(networkFirst(request));
  } else if (url.pathname.startsWith('/api/')) {
    // API calls: Network only (no caching for dynamic content)
    event.respondWith(fetch(request));
  } else {
    // Other assets: Cache first with runtime caching
    event.respondWith(cacheFirstRuntime(request));
  }
});

// Cache first strategy for static assets
async function cacheFirst(request) {
  try {
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
      return cachedResponse;
    }

    const networkResponse = await fetch(request);
    if (networkResponse.ok) {
      const cache = await caches.open(CACHE_NAME);
      cache.put(request, networkResponse.clone());
    }
    return networkResponse;
  } catch (error) {
    console.error('Cache first failed:', error);
    return new Response('Network error', { status: 503 });
  }
}

// Network first strategy for HTML pages
async function networkFirst(request) {
  try {
    const networkResponse = await fetch(request);
    if (networkResponse.ok) {
      const cache = await caches.open(CACHE_NAME);
      cache.put(request, networkResponse.clone());
    }
    return networkResponse;
  } catch (error) {
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
      return cachedResponse;
    }
    return new Response('Offline', { status: 503 });
  }
}

// Cache first with runtime caching
async function cacheFirstRuntime(request) {
  try {
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
      return cachedResponse;
    }

    const networkResponse = await fetch(request);
    if (networkResponse.ok) {
      const cache = await caches.open(RUNTIME_CACHE);
      cache.put(request, networkResponse.clone());
    }
    return networkResponse;
  } catch (error) {
    console.error('Runtime cache failed:', error);
    return new Response('Network error', { status: 503 });
  }
}
