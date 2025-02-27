import * as THREE from 'three'
import { Sky } from 'three/addons/objects/Sky.js'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { Timer } from 'three/addons/misc/Timer.js'
import GUI from 'lil-gui'

/**
 * Base
 */
// Debug
const gui = new GUI()

// Canvas
const canvas = document.querySelector('canvas.webgl')

// Scene
const scene = new THREE.Scene()

// House
const textureLoader = new THREE.TextureLoader()
const floorAlpha = textureLoader.load('floor/alpha.webp')

const floorDiffuse = textureLoader.load('floor/coast_sand_rocks_02_diff_1k.webp')
floorDiffuse.repeat.set(8, 8)
floorDiffuse.wrapS = THREE.RepeatWrapping
floorDiffuse.wrapT = THREE.RepeatWrapping
floorDiffuse.colorSpace = THREE.SRGBColorSpace

const floorArm = textureLoader.load('floor/coast_sand_rocks_02_arm_1k.webp')
floorArm.repeat.set(8, 8)
floorArm.wrapS = THREE.RepeatWrapping
floorArm.wrapT = THREE.RepeatWrapping

const floorNormMap = textureLoader.load('floor/coast_sand_rocks_02_nor_gl_1k.webp')
floorNormMap.repeat.set(8, 8)
floorNormMap.wrapS = THREE.RepeatWrapping
floorNormMap.wrapT = THREE.RepeatWrapping

const floorDisp = textureLoader.load('floor/coast_sand_rocks_02_nor_gl_1k.webp')
floorDisp.repeat.set(8, 8)
floorDisp.wrapS = THREE.RepeatWrapping
floorDisp.wrapT = THREE.RepeatWrapping

// floor
const floor = new THREE.Mesh(
    new THREE.PlaneGeometry(30, 30, 100, 100),
    new THREE.MeshStandardMaterial({
        alphaMap: floorAlpha,
        transparent: true,
        map: floorDiffuse,
        aoMap: floorArm,
        roughnessMap: floorArm,
        metalnessMap: floorArm,
        normalMap: floorNormMap,
        displacementMap: floorDisp,
        displacementScale: 0.3,
        displacementBias: -0.1
    })
)
floor.rotation.x = -Math.PI / 2
scene.add(floor)

gui.add(floor.material, 'displacementScale').min(0).max(1).step(0.001).name('displacementScale')
gui.add(floor.material, 'displacementBias').min(-1).max(1).step(0.001).name('displacementBias')

const house = new THREE.Group()
scene.add(house)

const wallDiffuse = textureLoader.load('wall/castle_brick_broken_06_diff_1k.webp')
wallDiffuse.colorSpace = THREE.SRGBColorSpace
const wallArm = textureLoader.load('wall/castle_brick_broken_06_arm_1k.webp')
const wallNormMap = textureLoader.load('wall/castle_brick_broken_06_nor_gl_1k.webp')

const walls = new THREE.Mesh(
    new THREE.BoxGeometry(4, 2.5, 4),
    new THREE.MeshStandardMaterial(
        {
            map: wallDiffuse,
            aoMap: wallArm,
            roughnessMap: wallArm,
            metalnessMap: wallArm,
            normalMap: wallNormMap
        }
    )
)
walls.position.y = 1.25
house.add(walls)

// roof
const roofDiffuse = textureLoader.load('roof/roof_slates_02_diff_1k.webp')
roofDiffuse.colorSpace = THREE.SRGBColorSpace
roofDiffuse.repeat.set(3, 1)
roofDiffuse.wrapS = THREE.RepeatWrapping
const roofArm = textureLoader.load('roof/roof_slates_02_arm_1k.webp')
roofArm.repeat.set(3, 1)
roofArm.wrapS = THREE.RepeatWrapping
const roofNormMap = textureLoader.load('roof/roof_slates_02_nor_gl_1k.webp')
roofNormMap.repeat.set(3, 1)
roofNormMap.wrapS = THREE.RepeatWrapping

const roof = new THREE.Mesh(
    new THREE.ConeGeometry(3.5, 1.5, 4),
    new THREE.MeshStandardMaterial(
        {
            map: roofDiffuse,
            aoMap: roofArm,
            roughnessMap: roofArm,
            metalnessMap: roofArm,
            normalMap: roofNormMap
        }
    )
)
roof.position.y = 2.5 + 0.75
roof.rotation.y = Math.PI / 4
house.add(roof)

// door
const doorDiffuse = textureLoader.load('door/color.webp')
doorDiffuse.colorSpace = THREE.SRGBColorSpace
const doorAO = textureLoader.load('door/ambientOcclusion.webp')
const doorRoughness = textureLoader.load('door/roughness.webp')
const doorMetalness = textureLoader.load('door/metalness.webp')
const doorNormMap = textureLoader.load('door/normal.webp')
const doorAlpha = textureLoader.load('door/alpha.webp')
const doorHeight = textureLoader.load('door/height.webp')

const door = new THREE.Mesh(
    new THREE.PlaneGeometry(2.2, 2.2, 100, 100),
    new THREE.MeshStandardMaterial({
        // color: 'red'
        transparent: true,
        map: doorDiffuse,
        aoMap: doorAO,
        roughnessMap: doorRoughness,
        metalnessMap: doorMetalness,
        normalMap: doorNormMap,
        alphaMap: doorAlpha,
        displacementMap: doorHeight,
        displacementScale: 0.15,
        displacementBias: -0.04
    })
)
door.position.y = 1
door.position.z = 2 + 0.001
house.add(door)

// bush
const bushDiffuse = textureLoader.load('bush/leaves_forest_ground_diff_1k.webp')
bushDiffuse.colorSpace = THREE.SRGBColorSpace
const bushArm = textureLoader.load('bush/leaves_forest_ground_arm_1k.webp')
const bushNormMap = textureLoader.load('bush/leaves_forest_ground_nor_gl_1k.webp')

const bushgeom = new THREE.SphereGeometry(1, 16, 16)
const bushmat = new THREE.MeshStandardMaterial({
    color: '#ccffcc',
    map: bushDiffuse,
    alphaMap: bushArm,
    roughnessMap: bushArm,
    metalnessMap: bushArm,
    normalMap: bushNormMap
})

const bush1 = new THREE.Mesh(bushgeom, bushmat)
bush1.scale.setScalar(0.5)
bush1.position.set(0.8, 0.2, 2.2)
bush1.rotation.x = -0.75

const bush2 = new THREE.Mesh(bushgeom, bushmat)
bush2.scale.setScalar(0.25)
bush2.position.set(1.4, 0.1, 2.1)
bush2.rotation.x = -0.75

const bush3 = new THREE.Mesh(bushgeom, bushmat)
bush3.scale.setScalar(0.4)
bush3.position.set(-0.8, 0.1, 2.2)
bush3.rotation.x = -0.75

const bush4 = new THREE.Mesh(bushgeom, bushmat)
bush4.scale.setScalar(0.15)
bush4.position.set(-1, 0.05, 2.6)
bush4.rotation.x = -0.75

house.add(bush1, bush2, bush3, bush4)

// graves
const graves = new THREE.Group()
scene.add(graves)

const graveDiffuse = textureLoader.load('grave/plastered_stone_wall_diff_1k.webp')
graveDiffuse.colorSpace = THREE.SRGBColorSpace
graveDiffuse.repeat.set(0.3, 0.4)
const graveArm = textureLoader.load('grave/plastered_stone_wall_arm_1k.webp')
graveArm.repeat.set(0.3, 0.4)
const graveNormMap = textureLoader.load('grave/plastered_stone_wall_nor_gl_1k.webp')
graveNormMap.repeat.set(0.3, 0.4)

const gravegeom = new THREE.BoxGeometry(0.6, 0.8, 0.2)
const gravemat = new THREE.MeshStandardMaterial({
    map: graveDiffuse,
    alphaMap: graveArm,
    roughnessMap: graveArm,
    metalnessMap: graveArm,
    normalMap: graveNormMap
})

for (let i = 0; i < 30; i++) {
    const angle = Math.random() * Math.PI * 2
    const radius = 4 + Math.random() * 4;
    const x = Math.sin(angle) * radius
    const z = Math.cos(angle) * radius

    const grave = new THREE.Mesh(gravegeom, gravemat)
    grave.position.x = x
    grave.position.z = z
    grave.position.y = Math.random() * 0.4

    grave.rotation.x = (Math.random() - 0.5) / 3
    grave.rotation.y = Math.random() * Math.PI

    graves.add(grave)
}

// Ambient light
const ambientLight = new THREE.AmbientLight('#86cdff', 0.275)
scene.add(ambientLight)

// Directional light
const directionalLight = new THREE.DirectionalLight('#86cdff', 1)
directionalLight.position.set(3, 2, -8)
scene.add(directionalLight)

const doorLight = new THREE.PointLight('#ff7d46', 5)
doorLight.position.set(0, 2.2, 2.5)
house.add(doorLight)


const ghost1 = new THREE.PointLight('#8800ff', 6)
const ghost2 = new THREE.PointLight('#ff0033', 6)
const ghost3 = new THREE.PointLight('#338811', 6)
scene.add(ghost1)
scene.add(ghost2)
scene.add(ghost3)

scene.fog = new THREE.FogExp2('#02343f', 0.1)
// scene.add(fog)

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
const camera = new THREE.PerspectiveCamera(45, sizes.width / sizes.height, 0.1, 100)
camera.position.x = 4
camera.position.y = 2
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

renderer.shadowMap.enabled = true
renderer.shadowMap.type = THREE.PCFSoftShadowMap

directionalLight.castShadow = true

ghost1.castShadow = true
ghost2.castShadow = true
ghost3.castShadow = true

walls.castShadow = true
walls.receiveShadow = true

roof.castShadow = true
roof.receiveShadow = true

door.castShadow = true
door.receiveShadow = true

floor.receiveShadow = true

graves.children.forEach((item) => {
    item.castShadow = true
    item.receiveShadow = true
})

directionalLight.shadow.mapSize.set(256, 256)
directionalLight.shadow.camera.top = 10
directionalLight.shadow.camera.right = 10
directionalLight.shadow.camera.bottom = -10
directionalLight.shadow.camera.left = -10
directionalLight.shadow.camera.near = 1
directionalLight.shadow.camera.far = 20

ghost1.shadow.mapSize.set(256, 256)
ghost1.shadow.camera.far = 10

ghost2.shadow.mapSize.set(256, 256)
ghost2.shadow.camera.far = 10

ghost3.shadow.mapSize.set(256, 256)
ghost3.shadow.camera.far = 10


const sky = new Sky()
scene.add(sky)
sky.material.uniforms['turbidity'].value = 10
sky.material.uniforms['rayleigh'].value = 3
sky.material.uniforms['mieCoefficient'].value = 0.1
sky.material.uniforms['mieDirectionalG'].value = 0.95
sky.material.uniforms['sunPosition'].value.set(0.3, -0.038, -0.95)
sky.scale.setScalar(1000)

/**
 * Animate
 */
const timer = new Timer()

const tick = () => {
    // Timer
    timer.update()
    const elapsedTime = timer.getElapsed()

    // ghost

    const ghostAngle = elapsedTime / 2
    ghost1.position.x = Math.sin(ghostAngle) * 5
    ghost1.position.z = Math.cos(ghostAngle) * 5
    ghost1.position.y = Math.sin(ghostAngle) * Math.sin(ghostAngle * 2.34) * Math.sin(ghostAngle * 3.45)

    const ghostAngle2 = -elapsedTime * 0.38
    ghost2.position.x = Math.sin(ghostAngle2) * 4
    ghost2.position.z = Math.cos(ghostAngle2) * 4
    ghost2.position.y = Math.sin(ghostAngle2) * Math.sin(ghostAngle * 2.34) * Math.sin(ghostAngle * 3.45)

    const ghostAngle3 = elapsedTime * 0.23
    ghost3.position.x = Math.sin(ghostAngle3) * 6
    ghost3.position.z = Math.cos(ghostAngle3) * 6
    ghost3.position.y = Math.sin(ghostAngle3) * Math.sin(ghostAngle * 2.34) * Math.sin(ghostAngle * 3.45)

    controls.update()

    // Render
    renderer.render(scene, camera)

    // Call tick again on the next frame
    window.requestAnimationFrame(tick)
}

tick()
