var scene3d = document.getElementById("viewer");

// SCENE

var scene = new THREE.Scene();

// CAMERA 

camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 100);
camera.position.x = 17;
camera.position.y = 12;
camera.position.z = 13;
camera.lookAt(scene.position);

// RENDERER

renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setClearColor(0x000, 1.0);
renderer.setSize(window.innerWidth, window.innerHeight);
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.autoRotate = true;
controls.update();

const size = 100;
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

function load_stl(name) {
    loader.load(
        name,
        function (geometry) {
            const mesh = new THREE.Mesh(geometry, material)
            //mesh.scale.set(0.05, 0.05, 0.05);
            scene.add(mesh)
        },
        (xhr) => {
            console.log((xhr.loaded / xhr.total) * 100 + '% loaded')
        },
        (error) => {
            console.log(error)
        }
    )
};

load_stl('/models/menger.stl');
// Subscribe to the event source at `uri` with exponential backoff reconnect.
function subscribe(uri) {
    var retryTime = 1;

    function connect(uri) {
        const events = new EventSource(uri);

        events.addEventListener("message", (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            const msg = JSON.parse(ev.data);
            console.log("LOAD STL HERE");
            console.log(msg);
        });

        events.addEventListener("open", () => {
            setConnectedStatus(true);
            console.log(`connected to event stream at ${uri}`);
            retryTime = 1;
        });

        events.addEventListener("error", () => {
            setConnectedStatus(false);
            events.close();

            let timeout = retryTime;
            retryTime = Math.min(64, retryTime * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => connect(uri), (() => timeout * 1000)());
        });
    }

    connect(uri);
}

function setConnectedStatus(status) {
    console.log(status);
    if (status) {
        scene.background = new THREE.Color(0, 0, 0);
    } else {
        scene.background = new THREE.Color(0.4, 0, 0);
    }
}

subscribe('/events');
animate();