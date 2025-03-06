const PHYSICS_TIMESTEP = 1/60;

// Add improved physics constants
const PHYSICS = {
    gravity: 9.81,
    drag: 0.02,
    turbulence: {
        scale: 0.2,
        frequency: 0.1
    }
};

// Add optimized physics calculations using TypedArrays
const particleBuffer = new Float32Array(1000 * 8); // pos(3) + vel(3) + age + life

self.onmessage = function(e) {
    const { particles, deltaTime, level } = e.data;
    const steps = Math.ceil(deltaTime / PHYSICS_TIMESTEP);

    for (let i = 0; i < steps; i++) {
        updateParticles(particles, PHYSICS_TIMESTEP, level);
    }

    self.postMessage(particles);
};

function updateParticles(particles, dt, level) {
    const batchSize = 50;
    const particleCount = particles.length;

    // Process in batches for better performance
    for (let i = 0; i < particleCount; i += batchSize) {
        const end = Math.min(i + batchSize, particleCount);
        updateParticleBatch(particles, i, end, dt, level);
    }
}

function updateParticleBatch(particles, start, end, dt, level) {
    const gravity = PHYSICS.gravity * (level >= 15 ? 1 + (level - 15) * 0.2 : 1);

    for (let i = start; i < end; i++) {
        const idx = i * 8;
        if (particleBuffer[idx + 6] >= particleBuffer[idx + 7]) continue; // Check age/life

        // Update velocity with optimized calculations
        particleBuffer[idx + 3] *= (1 - PHYSICS.drag);
        particleBuffer[idx + 4] = particleBuffer[idx + 4] * (1 - PHYSICS.drag) + gravity * dt;
        particleBuffer[idx + 5] *= (1 - PHYSICS.drag);

        // Update position
        particleBuffer[idx] += particleBuffer[idx + 3] * dt;
        particleBuffer[idx + 1] += particleBuffer[idx + 4] * dt;
        particleBuffer[idx + 2] += particleBuffer[idx + 5] * dt;

        // Update age
        particleBuffer[idx + 6] += dt;
    }
}

// Add simplex noise function for better turbulence
function simplex3D(x, y, z) {
    // ... simplified 3D noise implementation ...
    return {
        x: Math.sin(x * 10 + z) * 0.5,
        y: Math.cos(y * 8 + z) * 0.5,
        z: Math.sin((x + y) * 6 + z) * 0.5
    };
}
