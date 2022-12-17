const { invoke } = window.__TAURI__.tauri;

import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

let filepathEl;
let scene, renderer, camera;
let mesh;


function animate() {
  requestAnimationFrame(animate);

  const time = Date.now() * 0.001;

  if (mesh) {
    mesh.rotation.x = time * 0.25;
    mesh.rotation.y = time * 0.5;
  }

  renderer.render(scene, camera);
}


function load_mesh(mesh_data) {
  console.log("Loading mesh from data...");
  const vertices = []
  for (let i = 0; i < mesh_data.vertices.length; i++) {
    let v = mesh_data.vertices[i];
    vertices.push(v.x, v.y, v.z);
  }

  const indicies = []
  const normals = []
  const colours = []
  for (let i = 0; i < mesh_data.triangles.length; i++) {
    let t = mesh_data.triangles[i];
    indicies.push(t.indicies[0], t.indicies[1], t.indicies[2])
    normals.push(t.normal.x, t.normal.y, t.normal.z)
    colours.push(1, 0.2, 0.2)
  }

  const geometry = new THREE.BufferGeometry();
  geometry.setIndex(indicies);
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
  geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colours, 3));

  const material = new THREE.MeshPhongMaterial({
    side: THREE.DoubleSide,
    vertexColors: true
  });

  mesh = new THREE.Mesh(geometry, material);
  scene.add(mesh);
  console.log("Mesh added to scene!");
}

async function slice() {
  console.log("load file invoked");
  await invoke("load_file", { path: filepathEl.value })
    .then((response) => load_mesh(response));
}

window.addEventListener("DOMContentLoaded", () => {
  filepathEl = document.querySelector("#filepath")
  document
    .querySelector("#open")
    .addEventListener("click", () => slice());

  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(27, window.innerWidth / window.innerHeight, 1, 3500);
  camera.position.z = 64;
  renderer = new THREE.WebGLRenderer();
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.querySelector("#container").appendChild(renderer.domElement);

  const controls = new OrbitControls(camera, renderer.domElement);

  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x050505);

  const light = new THREE.HemisphereLight();
  scene.add(light);

  // const geometry = new THREE.BoxGeometry(1, 1, 1);
  // const material = new THREE.MeshNormalMaterial({ color: 0x00ff00 });
  // const cube = new THREE.Mesh(geometry, material);
  // scene.add(cube);

  animate();
});
