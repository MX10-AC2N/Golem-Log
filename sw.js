// sw.js
const CACHE_NAME = 'eos-guide-v1';
const urlsToCache = [
  '/',
  '/index.html',
  '/eos_guide_wasm.js', // Fichier généré par trunk
  '/eos_guide_wasm_bg.wasm' // Fichier généré par trunk
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request)
      .then((response) => {
        // Renvoie la réponse du cache ou fait une requête réseau
        return response || fetch(event.request);
      }
    )
  );
});
