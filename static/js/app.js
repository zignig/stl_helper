var scene3d = document.getElementById("viewer");

// SCENE

var scene = new THREE.Scene();

// CAMERA 

camera = new THREE.PerspectiveCamera(45,window.innerWidth/window.innerHeight, 0.1, 100);
camera.position.x = 17;
camera.position.y = 12;
camera.position.z = 13;
camera.lookAt(scene.position);

// RENDERER

renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setClearColor(0x000, 1.0);
renderer.setSize(window.innerWidth,window.innerHeight);
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.autoRotate = true;
controls.update();

const size = 10;
const divisions = 10;

const gridHelper = new THREE.GridHelper(size, divisions);
scene.add(gridHelper);

// light 1
light = new THREE.HemisphereLight(0xbbbbff, 0x444422);
light.position.set(0, 20, 0);
scene.add(light);

// light 2 
var spot1 = new THREE.SpotLight(0xffffff);
spot1.position.set(10, 100, -50);
scene.add(spot1);

// FINISH SCENE SETUP

scene3d.appendChild(renderer.domElement);
renderer.render(scene, camera);

function animate() {

    requestAnimationFrame(animate);

    // required if controls.enableDamping or controls.autoRotate are set to true
    controls.update();

    renderer.render(scene, camera);

}

const material = new THREE.MeshPhysicalMaterial({
    color: 0xb2ffc8,
    metalness: 0.25,
    roughness: 0.1,
    opacity: 1.0,
    clearcoat: 1.0,
    clearcoatRoughness: 0.25
})
const loader = new THREE.STLLoader()
loader.load(
    'models/cube.stl',
    function (geometry) {
        const mesh = new THREE.Mesh(geometry, material)
        mesh.scale.set(0.05, 0.05, 0.05);
        scene.add(mesh)
    },
    (xhr) => {
        console.log((xhr.loaded / xhr.total) * 100 + '% loaded')
    },
    (error) => {
        console.log(error)
    }
)

animate();