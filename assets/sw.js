var cacheName = 'simplenote-v2';

self.addEventListener('install', function (e) {
  self.skipWaiting(); // 立即接管，不等旧 SW
});

self.addEventListener('activate', function (e) {
  clients.claim();    // 接管所有已打开的标签
  e.waitUntil(
    caches.keys().then(function (names) {
      return Promise.all(
        names.map(function (name) { return caches.delete(name); })
      );
    })
  );
});

// 网络优先，失败才用缓存
self.addEventListener('fetch', function (e) {
  e.respondWith(
    fetch(e.request)
      .then(function (response) {
        var clone = response.clone();
        caches.open(cacheName).then(function (cache) {
          cache.put(e.request, clone);
        });
        return response;
      })
      .catch(function () {
        return caches.match(e.request);
      })
  );
});
