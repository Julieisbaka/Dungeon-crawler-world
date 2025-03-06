import { Camera3D } from './Camera.js';
import { Player } from './Player.js';
import { GridMesh } from './GridMesh.js';
import { BackdropMesh } from './BackdropMesh.js';

export class Scene3D {
    constructor(gl) {
        this.gl = gl;
        this.camera = new Camera3D();
        this.player = new Player(this.camera);
        this.createEnvironment();
    }

    createEnvironment() {
        // Create floor grid
        this.floor = new GridMesh(this.gl, {
            size: 100,
            divisions: 20,
            color: [0.2, 0.2, 0.3]
        });

        // Create backdrop walls
        this.walls = new BackdropMesh(this.gl, {
            width: 100,
            height: 40,
            depth: 100,
            color: [0.1, 0.1, 0.15]
        });
    }

    update(deltaTime) {
        this.player.update(deltaTime);
        this.camera.follow(this.player.position, deltaTime);
    }

    render() {
        const gl = this.gl;
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        const viewProjection = this.camera.getViewProjectionMatrix();

        // Render environment
        this.floor.render(viewProjection);
        this.walls.render(viewProjection);

        // Render active fireballs
        window.particleSystem?.render(viewProjection);
    }
}
