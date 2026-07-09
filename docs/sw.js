var cacheName = 'simplenote-v1';
var filesToCache = [
  './',
  './index.html',
  './eframe_template.js',
  './eframe_template_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  self.skipWaiting();
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Clear old caches */
self.addEventListener('activate', function (e) {
  e.waitUntil(
    caches.keys().then(function (names) {
      return Promise.all(
        names.filter(function (name) { return name !== cacheName; })
          .map(function (name) { return caches.delete(name); })
      );
    })
  );
});

/* Network first, fallback to cache */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    fetch(e.request)
      .then(function (response) {
        return caches.open(cacheName).then(function (cache) {
          cache.put(e.request, response.clone());
          return response;
        });
      })
      .catch(function () {
        return caches.match(e.request);
      })
  );
});
