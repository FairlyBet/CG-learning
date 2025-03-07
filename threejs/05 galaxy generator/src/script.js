import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import GUI from 'lil-gui'

/**
 * Base
 */
// Debug
const gui = new GUI({
    width: 300
})

// Canvas
const canvas = document.querySelector('canvas.webgl')

// Scene
const scene = new THREE.Scene()

let geometry;
let material;
let points;

function generateGalaxy() {
    console.log('generating galaxy...')

    if (points !== undefined) {
        geometry.dispose()
        material.dispose()
        scene.remove(points)
    }

    const valuesPerVertex = 3
    const positions = new Float32Array(parameters.count * valuesPerVertex)
    const colors = new Float32Array(parameters.count * valuesPerVertex)

    const colorInside = new THREE.Color(parameters.insideColor)
    const colorOutside = new THREE.Color(parameters.outsideColor)

    for (let i = 0; i < parameters.count; i++) {
        const i3 = i * valuesPerVertex

        const radius = Math.random() * parameters.radius
        const branchAngle = i % parameters.branches / parameters.branches * Math.PI * 2
        const spinAngle = radius * parameters.spin

        const randomX = Math.pow(Math.random(), parameters.randomnessPower)
            * (Math.random() < 0.5 ? -1 : 1) * parameters.randomness
        const randomY = Math.pow(Math.random(), parameters.randomnessPower)
            * (Math.random() < 0.5 ? -1 : 1) * parameters.randomness
        const randomZ = Math.pow(Math.random(), parameters.randomnessPower)
            * (Math.random() < 0.5 ? -1 : 1) * parameters.randomness

        positions[i3] = Math.cos(branchAngle + spinAngle) * radius + randomX
        positions[i3 + 1] = randomY;
        positions[i3 + 2] = Math.sin(branchAngle + spinAngle) * radius + randomZ;

        const mixedColor = colorInside.clone().lerp(colorOutside, radius / parameters.radius)
        colors[i3] = mixedColor.r
        colors[i3 + 1] = mixedColor.g
        colors[i3 + 2] = mixedColor.b
    }

    geometry = new THREE.BufferGeometry()
    geometry.setAttribute('position', new THREE.BufferAttribute(positions, valuesPerVertex))
    geometry.setAttribute('color', new THREE.BufferAttribute(colors, valuesPerVertex))

    material = new THREE.PointsMaterial({
        size: parameters.size,
        sizeAttenuation: true,
        depthWrite: false,
        vertexColors: true
        // blending: THREE.AdditiveBlending,
    })

    points = new THREE.Points(geometry, material)
    scene.add(points)
}

const parameters = {
    count: 10000,
    size: 0.001,
    radius: 3,
    branches: 3,
    spin: 1,
    randomness: 1,
    randomnessPower: 1,
    insideColor: '#ff6030',
    outsideColor: '#1b3984',
}

gui.add(parameters, 'count').min(1).max(100_000).step(100).onFinishChange(generateGalaxy)
gui.add(parameters, 'size').min(0.001).max(0.1).step(0.001).onFinishChange(generateGalaxy)
gui.add(parameters, 'radius').min(3).max(10).step(1).onFinishChange(generateGalaxy)
gui.add(parameters, 'branches').min(2).max(20).step(1).onFinishChange(generateGalaxy)
gui.add(parameters, 'spin').min(-10).max(10).step(0.1).onFinishChange(generateGalaxy)
gui.add(parameters, 'randomness').min(0).max(5).step(0.01).onFinishChange(generateGalaxy)
gui.add(parameters, 'randomnessPower').min(1).max(10).step(0.01).onFinishChange(generateGalaxy)
gui.addColor(parameters, 'insideColor').onFinishChange(generateGalaxy)
gui.addColor(parameters, 'outsideColor').onFinishChange(generateGalaxy)

generateGalaxy()

/**
 * Sizes
 */
const sizes = {
    width: window.innerWidth,
    height: window.innerHeight
}

window.addEventListener('resize', () => {
    // Update sizes
    sizes.width = window.innerWidth
    sizes.height = window.innerHeight

    // Update camera
    camera.aspect = sizes.width / sizes.height
    camera.updateProjectionMatrix()

    // Update renderer
    renderer.setSize(sizes.width, sizes.height)
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
})

/**
 * Camera
 */
// Base camera
const camera = new THREE.PerspectiveCamera(54, sizes.width / sizes.height, 0.1, 100)
camera.position.z = 5
scene.add(camera)

// Controls
const controls = new OrbitControls(camera, canvas)
controls.enableDamping = true

/**
 * Renderer
 */
const renderer = new THREE.WebGLRenderer({
    canvas: canvas
})
renderer.setSize(sizes.width, sizes.height)
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))

/**
 * Animate
 */
const clock = new THREE.Clock()

const tick = () => {
    const elapsedTime = clock.getElapsedTime()

    // Update controls
    controls.update()

    // Render
    renderer.render(scene, camera)

    // Call tick again on the next frame
    window.requestAnimationFrame(tick)
}

tick()