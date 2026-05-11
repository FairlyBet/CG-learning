/* tslint:disable */
/* eslint-disable */

/**
 * Generic PBR viewer renderer. JS-facing wasm-bindgen wrapper around `BaseRenderer`.
 *
 * Created by `OboxRendererPlugin` and registered with the viewer via `viewer.setRenderer()`.
 * To add custom render passes (custom materials, particles, etc.) make a separate WASM
 * crate that depends on `obox_renderer` as a Rust library (`rlib`) and extends
 * `BaseRenderer` directly. Generic JS-side pass plugins can't reach scene geometry / IBL /
 * framebuffer, which most non-trivial passes need.
 */
export class OboxRenderer {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns the camera eye position as a length-3 array [x, y, z].
     */
    get_camera_eye(): Float32Array;
    /**
     * Load a glTF model from a URL path. Replaces the currently visible scene.
     * Takes &self so wasm-bindgen does not hold an exclusive borrow during the async
     * network fetch — this allows render() to continue running while a model loads.
     */
    load_gltf(path: string): Promise<void>;
    /**
     * Whether the next render would do GL work (dirty OR camera moving).
     * JS plugins use this to gate their own per-frame work.
     */
    needs_render(): boolean;
    /**
     * Create the renderer.
     *
     * * `ibl_path` — IBL directory relative to `/static/` (e.g. `"environment_maps/studio4"`).
     * * `samples` — MSAA sample count for the offscreen render target. `None` or values
     *   ≥ 4 default to 4. Pass `Some(0)` to disable MSAA on weak / mobile devices.
     */
    static new(canvas: HTMLCanvasElement, ibl_path: string, samples?: number | null): Promise<OboxRenderer>;
    /**
     * Re-enable user orbit + zoom from the current camera pose.
     */
    release_camera(): void;
    /**
     * Render one frame. Called automatically each tick by OboxViewer's event delegation.
     */
    render(dt: number): void;
    /**
     * Request a re-render on the next frame. JS plugins call this when they mutate
     * state that affects rendering but isn't visible to the renderer's own dirty checks.
     * No-op cost; safe to call every frame.
     */
    request_redraw(): void;
    /**
     * Called automatically on canvas resize by OboxViewer's event delegation.
     */
    resize(width: number, height: number): void;
    /**
     * Bloom blur radius. Default: 0.1.
     */
    set_bloom_radius(v: number): void;
    /**
     * Bloom intensity. Default: 2.0.
     */
    set_bloom_strength(v: number): void;
    /**
     * Position camera directly (eye and look-at target as individual components).
     */
    set_camera(ex: number, ey: number, ez: number, tx: number, ty: number, tz: number): void;
    /**
     * Move camera along the configured path. `t∈[0,1]`. Locks user controls.
     */
    set_camera_animation(t: number): void;
    /**
     * Configure the orbit path used by `set_camera_animation`. All angles in degrees.
     * `radius` is distance from target on the XZ plane.
     */
    set_camera_path(angle_start: number, angle_end: number, radius: number, height_start: number, height_end: number, target_y: number): void;
    /**
     * Background clear color in linear RGB [0, 1]. Default: (0.1, 0.1, 0.1).
     */
    set_clear_color(r: number, g: number, b: number): void;
    /**
     * Near and far clip planes. Defaults: 0.1 / 1000.
     */
    set_clip_planes(near: number, far: number): void;
    /**
     * Camera exposure multiplier. Default: 1.0.
     */
    set_exposure(v: number): void;
    /**
     * Field of view in radians. Default: PI/4 (45°).
     */
    set_fov(fov: number): void;
    /**
     * Restrict vertical orbit to ±degrees around the equator (0–180).
     */
    set_latitude_range(degrees: number): void;
    /**
     * Orbit rotation speed in pixels-per-radian. Lower = more sensitive. Default: 200.
     */
    set_orbit_speed(speed: number): void;
    /**
     * Enable or disable panning. Default: false.
     */
    set_pan_enabled(enabled: boolean): void;
    /**
     * Inertia decay rate [0, 1]. Lower = more momentum. Default: 0.05.
     */
    set_smoothing_decay(decay: number): void;
    /**
     * Enable or disable rotation inertia (momentum after pointer release). Default: true.
     */
    set_smoothing_enabled(enabled: boolean): void;
    /**
     * Maximum zoom distance from the look-at center.
     */
    set_zoom_max(distance: number): void;
    /**
     * Minimum zoom distance from the look-at center.
     */
    set_zoom_min(distance: number): void;
    /**
     * Zero accumulated angular velocity from a previous user drag.
     */
    stop_camera_inertia(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_oboxrenderer_free: (a: number, b: number) => void;
    readonly oboxrenderer_get_camera_eye: (a: number) => [number, number];
    readonly oboxrenderer_load_gltf: (a: number, b: number, c: number) => any;
    readonly oboxrenderer_needs_render: (a: number) => number;
    readonly oboxrenderer_new: (a: any, b: number, c: number, d: number) => any;
    readonly oboxrenderer_release_camera: (a: number) => void;
    readonly oboxrenderer_render: (a: number, b: number) => void;
    readonly oboxrenderer_request_redraw: (a: number) => void;
    readonly oboxrenderer_resize: (a: number, b: number, c: number) => void;
    readonly oboxrenderer_set_bloom_radius: (a: number, b: number) => void;
    readonly oboxrenderer_set_bloom_strength: (a: number, b: number) => void;
    readonly oboxrenderer_set_camera: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly oboxrenderer_set_camera_animation: (a: number, b: number) => void;
    readonly oboxrenderer_set_camera_path: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly oboxrenderer_set_clear_color: (a: number, b: number, c: number, d: number) => void;
    readonly oboxrenderer_set_clip_planes: (a: number, b: number, c: number) => void;
    readonly oboxrenderer_set_exposure: (a: number, b: number) => void;
    readonly oboxrenderer_set_fov: (a: number, b: number) => void;
    readonly oboxrenderer_set_latitude_range: (a: number, b: number) => void;
    readonly oboxrenderer_set_orbit_speed: (a: number, b: number) => void;
    readonly oboxrenderer_set_pan_enabled: (a: number, b: number) => void;
    readonly oboxrenderer_set_smoothing_decay: (a: number, b: number) => void;
    readonly oboxrenderer_set_smoothing_enabled: (a: number, b: number) => void;
    readonly oboxrenderer_set_zoom_max: (a: number, b: number) => void;
    readonly oboxrenderer_set_zoom_min: (a: number, b: number) => void;
    readonly oboxrenderer_stop_camera_inertia: (a: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hdb80a4a930938138: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h9978d96831dc7e58: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hcefcff8205ffc6f8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hcefcff8205ffc6f8_2: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__ha1332a6dca94fd2c: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
