# Mountyhill — Website

Zwei Rust-Crates. `site` erzeugt statisches HTML, `viz` wird nach WebAssembly
übersetzt und rechnet im Browser. Kein Framework, kein Node, keine externen
Ressourcen zur Laufzeit.

Der Build besteht aus zwei Schritten. `cargo run -p site` allein erzeugt nur
das HTML — ohne den WASM-Schritt bleiben alle Grafiken leer.

```
cargo install wasm-pack             # einmalig
rustup target add wasm32-unknown-unknown

./build.sh                          # Linux, macOS, Git Bash
.\build.ps1                         # PowerShell
build.bat                           # cmd

cargo run --release -p site -- serve
```

Falls die Skripte nicht passen, sind es genau diese zwei Befehle:

```
wasm-pack build viz --release --target web --out-dir ../dist/wasm --out-name viz
cargo run --release -p site -- build
```

Meldet der Build `kein wasm in dist/wasm`, wurde der erste Schritt übersprungen.

`cargo run -p site -- build` allein baut nur das HTML neu und übernimmt ein
bereits vorhandenes `dist/wasm` — praktisch beim Arbeiten am Text.

## Was im Browser läuft

Jede Grafik ist ein `<canvas>` mit einem `data-viz`-Attribut. Das WASM-Modul
sucht sie beim Start, hängt an jede eine Szene und rendert alle aus einer
einzigen `requestAnimationFrame`-Schleife. Canvas außerhalb des Sichtfelds
werden übersprungen.

| Szene | Was passiert |
|---|---|
| `live` | Kurs entsteht laufend aus einem Zufallsprozess mit Drift, Regimewechseln und gelegentlichen Schocks. Fläche in Kachelfarbe, Linie in Off-White — dieselbe Form wie die Bildmarke |
| `equity` | Kapitalkurve zeichnet sich, darüber die Höchststandslinie, dazwischen der Rückgang schraffiert. Zeigt live den aktuellen und den maximalen Drawdown |
| `montecarlo` | 41 Pfade wachsen gleichzeitig, der Median hervorgehoben. Der schlechteste Verlauf wird mitgerechnet und ausgegeben |
| `markets` | 40 Sparklines, jede mit eigenem Prozess, alle 220 ms ein neuer Wert. Steigend und fallend unterschiedlich eingefärbt |
| `sizing` | Interaktiv: Der Regler skaliert die Positionsgröße, die Kurve wird in Echtzeit neu gerechnet. Macht sichtbar, dass mehr Risiko den Endwert nicht proportional erhöht, den Rückgang aber schon |

Die Zahlenanzeigen im HTML werden aus Rust heraus gesetzt — jede Szene liefert
über `Scene::readout` einen Wert, eine Nebenangabe und ein Vorzeichen.

## Farben

Alle acht Varianten sind im Einsatz. Jeder Abschnitt trägt eine Akzentfarbe;
beim Scrollen übernimmt die Marke im Kopf die Farbe des sichtbaren Abschnitts.
Das WASM-Modul liest `--tile` und `--light` alle 15 Frames aus dem
Computed Style — die Grafiken wechseln also mit, ohne dass Farben doppelt
gepflegt werden. Neue Farbe heißt: ein Eintrag in `ACCENTS` in `theme.rs`.

## Aufbau

| Pfad | Inhalt |
|---|---|
| `site/src/theme.rs` | Palette, erzeugt das Akzent-CSS |
| `site/src/components.rs` | Bildmarke, Signet, Kopf, Fuß |
| `site/src/site.rs` | Seitenaufbau |
| `site/src/main.rs` | Build nach `dist/`, Vorschau-Server |
| `viz/src/rng.rs` | Xorshift, Zufallspfade, Drawdown-Berechnung |
| `viz/src/scenes.rs` | die fünf Szenen |
| `viz/src/lib.rs` | Canvas-Anbindung, Renderschleife, Farbabgleich |

## Veröffentlichen

`dist/` ist ein statisches Verzeichnis. Der Workflow unter
`.github/workflows/pages.yml` installiert wasm-pack, baut und veröffentlicht
auf GitHub Pages. Bei Cloudflare Pages als Build-Befehl `./build.sh` und als
Ausgabeverzeichnis `dist` eintragen.

Wichtig beim eigenen Hosting: `.wasm` muss als `application/wasm` ausgeliefert
werden, sonst verweigert der Browser das Modul. GitHub Pages und Cloudflare
machen das von sich aus.

## Vor dem Livegang

- **Impressum und Datenschutz** enthalten Platzhalter für Anschrift, Register
  und Umsatzsteuer-ID. Diese Angaben sind rechtlich verbindlich.
- **Alle Kurven sind simuliert** und als solche im Fußtext gekennzeichnet. Das
  ist bewusst so: Sobald echte Wertentwicklungen gezeigt werden, greifen
  Kennzeichnungspflichten, die vorher zu klären sind.
- **Schrift**: derzeit die Systemschrift. Für den markengetreuen Auftritt eine
  Grotesk wie Inter selbst hosten und in `style.css` voranstellen.
