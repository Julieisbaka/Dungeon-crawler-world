class FireballMesh {
    constructor(gl) {
        this.gl = gl;
        this.initBuffers();
        this.loadTextures();
        this.createProgram();
    }

    async loadTextures() {
        this.textures = {
            diffuse: await this.loadTexture('textures/fireball_diffuse.png'),
            normal: await this.loadTexture('textures/fireball_normal.png'),
            emissive: await this.loadTexture('textures/fireball_emissive.png')
        };
    }

    createProgram() {
        const gl = this.gl;

        // Load and compile shaders
        const vertexShader = loadShader(gl, gl.VERTEX_SHADER, 'shaders/fireball.vert.glsl');
        const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, 'shaders/fireball.frag.glsl');

        // Create program and link shaders
        this.program = gl.createProgram();
        gl.attachShader(this.program, vertexShader);
        gl.attachShader(this.program, fragmentShader);
        gl.linkProgram(this.program);

        // Get attribute and uniform locations
        this.locations = {
            attributes: {
                position: gl.getAttribLocation(this.program, 'position'),
                normal: gl.getAttribLocation(this.program, 'normal'),
                uv: gl.getAttribLocation(this.program, 'uv'),
                instancePosition: gl.getAttribLocation(this.program, 'instancePosition'),
                instanceRotation: gl.getAttribLocation(this.program, 'instanceRotation')
            },
            uniforms: {
                projectionMatrix: gl.getUniformLocation(this.program, 'projectionMatrix'),
                viewMatrix: gl.getUniformLocation(this.program, 'viewMatrix'),
                time: gl.getUniformLocation(this.program, 'time'),
                level: gl.getUniformLocation(this.program, 'level'),
                color: gl.getUniformLocation(this.program, 'color')
            }
        };
    }

    render(camera, instances, level) {
        const gl = this.gl;

        gl.useProgram(this.program);

        // Update uniforms
        gl.uniformMatrix4fv(this.locations.uniforms.projectionMatrix, false, camera.projectionMatrix);
        gl.uniformMatrix4fv(this.locations.uniforms.viewMatrix, false, camera.viewMatrix);
        gl.uniform1f(this.locations.uniforms.time, performance.now() * 0.001);
        gl.uniform1f(this.locations.uniforms.level, level);

        // Bind textures
        this.bindTextures();

        // Update instance data
        this.updateInstanceData(instances);

        // Draw instances
        gl.drawElementsInstanced(gl.TRIANGLES, this.indexCount, gl.UNSIGNED_SHORT, 0, instances.length);
    }
}
