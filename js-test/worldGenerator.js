class WorldGenerator {
    constructor(scene, textures) {
        this.scene = scene;
        this.textures = textures;
        this.config = this.loadConfig();
        this.generateTerrain();
    }

    loadConfig() {
        // Load from graphics.json
        return {
            segments: 128,
            displacementScale: 2.0,
            noiseScale: 0.1
        };
    }

    generateTerrain() {
        const size = 100;
        const geometry = new THREE.PlaneGeometry(
            size,
            size,
            this.config.segments,
            this.config.segments
        );

        // Generate height map using Perlin noise
        for (let i = 0; i < geometry.vertices.length; i++) {
            const vertex = geometry.vertices[i];
            vertex.z = noise.perlin2(
                vertex.x * this.config.noiseScale,
                vertex.y * this.config.noiseScale
            ) * 10;
        }

        geometry.computeVertexNormals();

        // Create terrain material with texture blending
        const material = new THREE.MeshStandardMaterial({
            map: this.textures.grass,
            normalMap: this.textures.rock,
            displacementMap: this.textures.dirt,
            displacementScale: this.config.displacementScale
        });

        this.terrain = new THREE.Mesh(geometry, material);
        this.terrain.rotation.x = -Math.PI / 2;
        this.scene.add(this.terrain);

        // Add environment objects
        this.addTrees();
        this.addRocks();
    }

    addTrees() {
        // Simple tree geometry
        const treeGeometry = new THREE.CylinderGeometry(0, 1.5, 4);
        const treeMaterial = new THREE.MeshStandardMaterial({ color: 0x2d5a27 });

        for (let i = 0; i < 20; i++) {
            const tree = new THREE.Mesh(treeGeometry, treeMaterial);
            const x = Math.random() * 80 - 40;
            const z = Math.random() * 80 - 40;
            const y = this.getHeightAtPosition(x, z);
            tree.position.set(x, y, z);
            this.scene.add(tree);
        }
    }

    getHeightAtPosition(x, z) {
        // Calculate height using Perlin noise
        const scale = this.config.noiseScale;
        return noise.perlin2(x * scale, z * scale) * 10;
    }
}
