const { invoke } = window.__TAURI__.tauri;

import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

let filepathEl;
let scene, renderer, camera, controls;
let mesh;


function animate() {
  requestAnimationFrame(animate);

  const time = Date.now() * 0.001;

  // if (mesh) {
  //   mesh.rotation.x = time * 0.25;
  //   mesh.rotation.y = time * 0.5;
  // }

  controls.update();

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
  geometry.computeBoundingBox();

  const material = new THREE.MeshPhongMaterial({
    side: THREE.DoubleSide,
    vertexColors: true
  });

  mesh = new THREE.Mesh(geometry, material);

  // Our verticies are all world-relative, and we want to ensure there's at least one point intersecting (0,0,0)
  let point = mesh_data.vertices[0];
  mesh.translateX(-point.x);
  mesh.translateY(-point.y);
  mesh.translateZ(-point.z);

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
  scene.background = new THREE.Color(0xbfe3dd);

  camera = new THREE.PerspectiveCamera(90, window.innerWidth / window.innerHeight, 0.1, 2000);
  camera.position.set(5, 2, 8);

  renderer = new THREE.WebGLRenderer();
  renderer.setSize(window.innerWidth - 100, window.innerHeight - 100);
  document.querySelector("#container").appendChild(renderer.domElement);

  controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(0, 0.5, 0);
  controls.update();
  controls.enableDamping = true;

  const light = new THREE.HemisphereLight();
  scene.add(light);

  const helper = new THREE.GridHelper(100, 100);
  helper.material.opacity = 0.25;
  helper.material.transparent = true;
  scene.add(helper);

  animate();
});
