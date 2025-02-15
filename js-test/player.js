class Player {
    constructor(camera) {
        this.camera = camera;
        this.moveSpeed = 0.1;
        this.keys = {};

        window.addEventListener('keydown', (e) => this.keys[e.key] = true);
        window.addEventListener('keyup', (e) => this.keys[e.key] = false);

        document.addEventListener('mousemove', (e) => this.onMouseMove(e));
        document.body.requestPointerLock = document.body.requestPointerLock || document.body.mozRequestPointerLock;
        document.addEventListener('click', () => document.body.requestPointerLock());
    }

    update() {
        if (this.keys['w']) this.camera.position.z -= this.moveSpeed;
        if (this.keys['s']) this.camera.position.z += this.moveSpeed;
        if (this.keys['a']) this.camera.position.x -= this.moveSpeed;
        if (this.keys['d']) this.camera.position.x += this.moveSpeed;
    }

    onMouseMove(event) {
        if (document.pointerLockElement === document.body) {
            this.camera.rotation.y -= event.movementX * 0.002;
            this.camera.rotation.x -= event.movementY * 0.002;
            this.camera.rotation.x = Math.max(-Math.PI/2, Math.min(Math.PI/2, this.camera.rotation.x));
        }
    }
}
