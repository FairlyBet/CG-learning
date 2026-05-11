# @obox-systems/obox-renderer

Default WASM PBR renderer plugin for [`obox-viewer`][core]. Ships a
WebGL2/wasm-bindgen renderer (`OboxRendererPlugin`) and a zero-config Web
Component (`<obox-viewer>`).

[core]: https://www.npmjs.com/package/obox-viewer

This package is split out from `obox-viewer` itself so consumers who don't
use the default renderer (e.g. those bundling their own WASM via a custom
plugin) don't ship the ~860 KB `.wasm` payload as dead bytes.

## Install

```bash
npm install obox-viewer @obox-systems/obox-renderer
```

`obox-viewer` is a peer dependency.

## Usage from JS

```js
import { OboxViewer } from 'obox-viewer';
import { OboxRendererPlugin } from '@obox-systems/obox-renderer';

const viewer = new OboxViewer({ container: '#root', assetsUrl: '/assets/' });

viewer.use(new OboxRendererPlugin({
  iblPath:  'environment_maps/studio4',  // relative to /static/ on the server
  modelUrl: 'gltf/my_model.glb',         // loaded before render loop starts
}));

await viewer.init();
```

After init, the renderer is accessible for runtime calls:

```js
const r = viewer.getPlugin('obox-renderer').renderer;
await r.load_gltf('gltf/other.glb');
r.set_camera(0, 1, 3,  0, 0, 0);
r.set_exposure(1.2);
```

### Web Component (zero-config)

For HTML-only integrations:

```html
<script type="module">
  import '@obox-systems/obox-renderer/element';
</script>

<obox-viewer
  src="gltf/my_model.glb"
  ibl="environment_maps/studio4"
  style="width:600px;height:600px"></obox-viewer>
```

The element auto-instantiates `OboxRendererPlugin`, fires a `ready` event
after init, and exposes `.viewer` and `.renderer` getters for JS access.

### Plugin options

```ts
new OboxRendererPlugin({
  wasmUrl?:  string,           // override the wasm-bindgen JS shim URL
  iblPath?:  string,           // default 'environment_maps/studio4'
  modelUrl?: string,
  msaaSamples?: number,
  camera?: {
    fov?: number, eye?: [number,number,number], center?: [number,number,number],
    near?: number, far?: number, speed?: number, smoothing?: boolean,
    decay?: number, pan?: boolean, latitudeRange?: number,
    zoom?: { min?: number, max?: number },
  },
  renderer?: {
    exposure?: number,
    backgroundColor?: [number,number,number],
    bloom?: { strength?: number, radius?: number },
  },
});
```

## Building from source

`npm run build` invokes Vite, which drives `wasm-pack` via a small
plugin in [`vite.config.js`](./vite.config.js). The `prepack` script
runs the same command automatically when you `npm pack` this workspace.

```bash
# from the monorepo root
npm run build -w @obox-systems/obox-renderer

# or from this directory
npm run build
# or, bypassing Vite, directly:
wasm-pack build --target web --out-dir pkg --release
```

Outputs `pkg/obox_renderer.js` and `pkg/obox_renderer_bg.wasm` (both
git-ignored). Requires a Rust toolchain with the `wasm32-unknown-unknown`
target and `wasm-pack` ≥ 0.12.

## Usage as a Rust library

Downstream Rust crates can depend on this crate to extend the base renderer:

```toml
[dependencies]
obox_renderer = { git = "https://github.com/obox-systems/obox-viewer.git" }
```

Cargo's git resolver scans the repo for matching `Cargo.toml` files and
will find this crate at `packages/renderer/`.

```rust
use obox_renderer::BaseRenderer;

pub struct MyRenderer {
    base: BaseRenderer,
}

impl MyRenderer {
    pub async fn new(canvas: HtmlCanvasElement, ibl_path: String) -> Result<Self, JsValue> {
        let base = BaseRenderer::new(&canvas, &ibl_path).await?;
        Ok(Self { base })
    }

    pub fn render(&mut self, dt: f32) {
        self.base.render_frame(dt);  // standard PBR
        self.my_extra_pass(dt);      // custom rendering with full GL/camera/scene access
    }
}
```

`BaseRenderer` exposes (all `pub`): `gl`, `render_frame(dt)`, `resize(w, h)`,
`load_gltf(path)`, `scene()`, `set_scene(...)`, `pipeline()` /
`pipeline_mut()`, `request_redraw()` / `needs_render()`, plus all the camera
/ orbit / renderer setters. The `Pipeline` struct (cgtools `Renderer`,
`Camera`, swap buffer, post-process passes) is `pub` with `pub` fields, so
downstream crates can drive cgtools internals directly (custom materials,
shadow baking, etc.).

## Asset loading — known limitation

cgtools (the underlying rendering library) constructs asset URLs as:

```
{origin}/static/{path}
```

This means `load_gltf("gltf/model.glb")` always fetches from the **same
origin** at `/static/gltf/model.glb`. Loading from a CDN or arbitrary URL is
not supported today.

**Workaround:** serve a reverse proxy or use a local junction/symlink that
maps `/static/` to your asset directory.

**Future:** when cgtools adds a bytes-based loader API
(`load_gltf_bytes(Uint8Array)`), this can be fixed without API changes — the
JS caller would `fetch()` from anywhere and pass the bytes in. The same
applies to IBL textures.

## IBL path convention

`iblPath` is relative to `/static/` on the server. cgtools expects:

```
/static/{iblPath}/diffuse.hdr
/static/{iblPath}/specular_*.hdr   (mip levels)
```

Default: `environment_maps/studio4`.

## License

MIT — see [LICENSE](./LICENSE).
