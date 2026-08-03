(function () {
  var root = document.documentElement;

  function applyAccent(key) {
    if (root.classList.contains("t-" + key)) return;
    root.className = root.className.replace(/\bt-[a-z]+\b/g, "").trim() + " t-" + key;
  }

  var themed = document.querySelectorAll("[data-accent]");
  if (themed.length && "IntersectionObserver" in window) {
    var accentObserver = new IntersectionObserver(function (entries) {
      var best = null;
      entries.forEach(function (e) {
        if (!e.isIntersecting) return;
        if (!best || e.intersectionRatio > best.intersectionRatio) best = e;
      });
      if (best) applyAccent(best.target.getAttribute("data-accent"));
    }, { rootMargin: "-45% 0px -45% 0px", threshold: [0, 0.01, 0.5, 1] });
    Array.prototype.forEach.call(themed, function (el) { accentObserver.observe(el); });
  }

  var reveals = document.querySelectorAll(".reveal");
  if ("IntersectionObserver" in window) {
    var revealObserver = new IntersectionObserver(function (entries, obs) {
      entries.forEach(function (e) {
        if (!e.isIntersecting) return;
        e.target.classList.add("in");
        obs.unobserve(e.target);
      });
    }, { rootMargin: "0px 0px -10% 0px", threshold: 0.06 });
    Array.prototype.forEach.call(reveals, function (el) { revealObserver.observe(el); });
  } else {
    Array.prototype.forEach.call(reveals, function (el) { el.classList.add("in"); });
  }

  var toggle = document.querySelector(".nav-toggle");
  if (toggle) {
    toggle.addEventListener("click", function () {
      document.body.classList.toggle("nav-open");
    });
    Array.prototype.forEach.call(document.querySelectorAll(".site-nav a"), function (a) {
      a.addEventListener("click", function () { document.body.classList.remove("nav-open"); });
    });
  }

  var onScroll = function () {
    document.body.classList.toggle("scrolled", window.scrollY > 40);
  };
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();
})();
