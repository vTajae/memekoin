// Simple service worker for Meme Koin Trading Platform
// This provides basic caching for better performance

const CACHE_NAME = 'koin-v1';
const urlsToCache = [
  '/',
  '/pkg/koin.js',
  '/pkg/koin.wasm',
  '/pkg/koin_bg.wasm',
  '/pkg/koin.css'
];

self.addEventListener('install', function(event) {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) {
        return cache.addAll(urlsToCache);
      })
  );
});

self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request)
      .then(function(response) {
        // Return cached version or fetch from network
        return response || fetch(event.request);
      }
    )
  );
});
