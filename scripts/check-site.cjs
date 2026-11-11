// Checks the shipped static pages, links, metadata and demo manifest without a browser.
const fs = require("node:fs"),
  path = require("node:path"),
  assert = require("node:assert/strict");
const root = path.resolve(__dirname, "../dist/site"),
  origin = "https://mpjhorner.github.io/Wow-Whatsapp-Addon/";
const pages = ["index.html", "install/index.html"];
const contents = new Map(
  pages.map((name) => [name, fs.readFileSync(path.join(root, name), "utf8")]),
);
for (const [name, html] of contents) {
  assert.equal(
    (html.match(/<h1\b/g) || []).length,
    1,
    `${name}: one primary heading`,
  );
  const descriptionTag = html.match(/<meta\b[^>]*name="description"[^>]*>/)?.[0];
  const description = descriptionTag?.match(/content="([^"]*)"/)?.[1] || "";
  assert(description.length >= 60 && description.length <= 180, `${name}: description`);
  const canonicalTag = html.match(/<link\b[^>]*rel="canonical"[^>]*>/)?.[0] || "";
  assert(
    canonicalTag.includes(
      `href="${origin}${name === "index.html" ? "" : "install/"}"`,
    ),
    `${name}: canonical`,
  );
  const ids = [...html.matchAll(/\bid="([^"]+)"/g)].map((match) => match[1]);
  assert.equal(new Set(ids).size, ids.length, `${name}: duplicate IDs`);
  const schemas = [...html.matchAll(
    /<script\b[^>]*type="application\/ld\+json"[^>]*>([\s\S]*?)<\/script>/g,
  )];
  assert(schemas.length > 0, `${name}: structured metadata`);
  for (const [, json] of schemas) {
    const data = JSON.parse(json);
    assert.equal(data["@context"], "https://schema.org");
    assert(
      !JSON.stringify(data).includes("aggregateRating"),
      "No invented ratings",
    );
  }
  for (const [, value] of html.matchAll(/(?:href|src)="([^"]+)"/g)) {
    if (/^(https?:|mailto:|data:)/.test(value)) continue;
    const url = new URL(
      value,
      origin + (name === "index.html" ? "" : "install/"),
    );
    assert.equal(url.origin, new URL(origin).origin);
    assert(url.pathname.startsWith("/Wow-Whatsapp-Addon/"));
    let target = decodeURIComponent(
      url.pathname.slice("/Wow-Whatsapp-Addon/".length),
    );
    if (!target || target.endsWith("/")) target += "index.html";
    assert(fs.existsSync(path.join(root, target)), `${name}: missing ${value}`);
    if (url.hash && target.endsWith(".html")) {
      const destination =
        contents.get(target) ||
        fs.readFileSync(path.join(root, target), "utf8");
      assert(
        destination.includes(`id="${decodeURIComponent(url.hash.slice(1))}"`),
        `${name}: broken anchor ${value}`,
      );
    }
  }
}
for (const [, url] of fs
  .readFileSync(path.join(root, "styles.css"), "utf8")
  .matchAll(/url\(['"]?([^'"\)]+)['"]?\)/g)) {
  assert(fs.existsSync(path.join(root, url)), `Missing CSS asset ${url}`);
}
const demos = JSON.parse(
  fs.readFileSync(path.resolve(__dirname, "../site/demos.json"), "utf8"),
);
assert.equal(demos.length, 3);
for (const demo of demos) {
  assert.equal(demo.messages.length, 6);
  assert(fs.existsSync(path.join(root, demo.image)));
}
assert(fs.existsSync(path.join(root, "assets/social-card.png")));
assert(
  fs
    .readFileSync(path.join(root, "sitemap.xml"), "utf8")
    .includes(origin + "install/"),
);
assert(!/\{\{[A-Z_]+\}\}/.test(contents.get("index.html")));
console.log(
  "Static site checks passed: page metadata, schema, links, anchors, assets, sitemap and three complete demos.",
);
