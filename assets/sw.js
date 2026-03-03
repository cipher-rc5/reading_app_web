const cacheName = 'reading-app-pwa';
const filesToCache = ['./', './index.html', './reading_app.js', './reading_app_bg.wasm'];
const devHosts = new Set(['127.0.0.1', 'localhost']);
const isDevEnv = devHosts.has(self.location.hostname);

const passthroughFetch = (event) => {
  event.respondWith(fetch(event.request));
};

self.addEventListener('install', (event) => {
  if (isDevEnv) {
    self.skipWaiting();
    return;
  }

  event.waitUntil(
    caches.open(cacheName).then((cache) => {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener('activate', (event) => {
  if (isDevEnv) {
    event.waitUntil(self.clients.claim());
    return;
  }

  event.waitUntil(
    caches.keys().then((cacheNames) =>
      Promise.all(cacheNames.map((name) => {
        if (name !== cacheName) {
          return caches.delete(name);
        }
        return Promise.resolve(true);
      }))
    )
  );
});

if (isDevEnv) {
  self.addEventListener('fetch', passthroughFetch);
} else {
  self.addEventListener('fetch', (event) => {
    event.respondWith(
      caches.match(event.request).then((response) => {
        return response || fetch(event.request);
      })
    );
  });
}
