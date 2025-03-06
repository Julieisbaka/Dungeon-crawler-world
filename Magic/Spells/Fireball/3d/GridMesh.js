export class GridMesh {
    constructor(gl, options) {
        this.gl = gl;
        this.size = options.size;
        this.divisions = options.divisions;
        this.color = options.color;
        this.initBuffers();
    }

    initBuffers() {
        const vertices = [];
        const step = this.size / this.divisions;

        // Create grid lines
        for (let i = 0; i <= this.divisions; i++) {
            const x = (i * step) - this.size / 2;
            vertices.push(
                x, 0, -this.size / 2,
                x, 0, this.size / 2,
                -this.size / 2, 0, x,
                this.size / 2, 0, x
            );
        }

        this.vertexBuffer = this.gl.createBuffer();
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.vertexBuffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(vertices), this.gl.STATIC_DRAW);
        this.vertexCount = vertices.length / 3;
    }

    render(viewProjection) {
        // ... render grid lines ...
    }
}
