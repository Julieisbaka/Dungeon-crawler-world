class Player {
    constructor(camera) {
        this.position = { x: 0, y: 1.8, z: 0 };
        this.rotation = { x: 0, y: 0 };
        this.velocity = { x: 0, y: 0, z: 0 };
        this.camera = camera;

        this.setupControls();
    }

    setupControls() {
        document.addEventListener('mousemove', (e) => {
            if (document.pointerLockElement) {
                this.rotation.y -= e.movementX * 0.002;
                this.rotation.x = Math.max(-Math.PI/2,
                    Math.min(Math.PI/2, this.rotation.x - e.movementY * 0.002));
            }
        });

        document.addEventListener('click', (e) => {
            if (!document.pointerLockElement) {
                document.body.requestPointerLock();
            } else {
                this.castFireball();
            }
        });

        this.keys = new Set();
        document.addEventListener('keydown', e => this.keys.add(e.code));
        document.addEventListener('keyup', e => this.keys.delete(e.code));
    }

    update(deltaTime) {
        // Movement
        const speed = 5;
        const direction = { x: 0, z: 0 };

        if (this.keys.has('KeyW')) direction.z -= 1;
        if (this.keys.has('KeyS')) direction.z += 1;
        if (this.keys.has('KeyA')) direction.x -= 1;
        if (this.keys.has('KeyD')) direction.x += 1;

        // Apply rotation to movement
        const cos = Math.cos(this.rotation.y);
        const sin = Math.sin(this.rotation.y);
        this.velocity.x = (direction.x * cos - direction.z * sin) * speed;
        this.velocity.z = (direction.x * sin + direction.z * cos) * speed;

        // Update position
        this.position.x += this.velocity.x * deltaTime;
        this.position.z += this.velocity.z * deltaTime;

        // Update camera target
        this.camera.setTarget(this.position, this.rotation);
    }

    castFireball() {
        const direction = this.camera.getForwardVector();
        const startPos = { ...this.position };
        const targetPos = {
            x: startPos.x + direction.x * 100,
            y: startPos.y + direction.y * 100,
            z: startPos.z + direction.z * 100
        };

        castFireball(
            startPos.x, startPos.y, startPos.z,
            targetPos.x, targetPos.y, targetPos.z,
            window.currentSpellLevel
        );
    }
}
