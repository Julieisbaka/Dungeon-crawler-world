class Camera3D {
    constructor() {
        this.position = { x: 0, y: 2, z: 5 };
        this.target = { x: 0, y: 0, z: 0 };
        this.up = { x: 0, y: 1, z: 0 };

        this.fov = 75;
        this.aspect = window.innerWidth / window.innerHeight;
        this.near = 0.1;
        this.far = 1000;

        this.updateProjectionMatrix();
    }

    updateProjectionMatrix() {
        this.projectionMatrix = mat4.perspective(
            mat4.create(),
            this.fov * Math.PI / 180,
            this.aspect,
            this.near,
            this.far
        );
    }

    setTarget(position, rotation) {
        this.target.x = position.x;
        this.target.y = position.y;
        this.target.z = position.z;

        // Calculate camera position based on player rotation
        const distance = 0; // First person camera
        this.position.x = position.x - Math.sin(rotation.y) * distance;
        this.position.y = position.y + Math.sin(rotation.x) * distance;
        this.position.z = position.z - Math.cos(rotation.y) * distance;
    }

    getViewProjectionMatrix() {
        const viewMatrix = mat4.lookAt(
            mat4.create(),
            [this.position.x, this.position.y, this.position.z],
            [this.target.x, this.target.y, this.target.z],
            [this.up.x, this.up.y, this.up.z]
        );

        return mat4.multiply(mat4.create(), this.projectionMatrix, viewMatrix);
    }

    getForwardVector() {
        const forward = {
            x: this.target.x - this.position.x,
            y: this.target.y - this.position.y,
            z: this.target.z - this.position.z
        };
        const length = Math.sqrt(
            forward.x * forward.x +
            forward.y * forward.y +
            forward.z * forward.z
        );
        return {
            x: forward.x / length,
            y: forward.y / length,
            z: forward.z / length
        };
    }
}
