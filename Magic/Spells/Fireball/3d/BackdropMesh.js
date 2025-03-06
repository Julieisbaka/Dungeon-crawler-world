export class BackdropMesh {
    constructor(gl, options) {
        this.gl = gl;
        this.width = options.width;
        this.height = options.height;
        this.depth = options.depth;
        this.color = options.color;
        this.initBuffers();
    }

    initBuffers() {
        // Create room walls
        const vertices = [
            // Back wall
            -this.width/2, 0, -this.depth/2,
            -this.width/2, this.height, -this.depth/2,
            this.width/2, this.height, -this.depth/2,
            this.width/2, 0, -this.depth/2,
            // Side walls...
        ];

        this.vertexBuffer = this.gl.createBuffer();
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.vertexBuffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(vertices), this.gl.STATIC_DRAW);
    }

    render(viewProjection) {
        // ... render walls ...
    }
}
