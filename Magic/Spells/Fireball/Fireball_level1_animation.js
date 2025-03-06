/**
 * Level 1 Fireball Animation
 * Pure HTML/CSS/JS implementation without game engine dependencies
 */

// Constants for the fireball animation
const FIREBALL_CONFIG = {
  // Core animation properties
  duration: 1200, // milliseconds

  // Appearance
  size: {
    initial: 30, // pixels
    final: 50,   // pixels at impact
  },
  speed: 400, // pixels per second

  // Colors
  colors: {
    core: "#FF6B00", // bright orange
    glow: "#FF3800", // deep red
    trail: ["#FF9933", "#FF6600", "#CC3300"]
  },
  special: {
    dragonHeads: false,    // Level 16: Dragon heads appear
    vortex: false,         // Level 17: Spinning vortex effect
    multiCore: false,      // Level 18: Multiple cores
    phoenixWings: false,   // Level 19: Phoenix wing effects
    timeDistortion: false  // Level 20: Time distortion trails
  },
  particles: {
    count: 12,          // Reduced base count
    maxParticles: 100,  // Cap total particles
    batchSize: 20,      // Batch processing size
    lifetime: 800,      // Particle lifetime in ms
    baseSize: {
      min: 4,
      max: 12
    },
    spread: 45,         // Spread angle in degrees
    speed: {
      min: 50,
      max: 150
    },
    opacity: {
      start: 0.8,
      end: 0
    }
  },
  perspective: {
    depth: 100,         // 3D depth
    rotation: {
      x: 0.1,          // Rotation speeds
      y: 0.2,
      z: 0.15
    }
  }
};

const FIREBALL_LEVELS = {
  // Levels 1-5: Orange to Red progression
  1: { core: "#FF6B00", glow: "#FF3800", trail: ["#FF9933", "#FF6600", "#CC3300"] },
  5: { core: "#FF4400", glow: "#FF2200", trail: ["#FF7733", "#FF4400", "#CC2200"] },
  // Levels 6-10: Red to Blue progression
  10: { core: "#0066FF", glow: "#0033FF", trail: ["#3399FF", "#0066FF", "#0033CC"] },
  // Levels 11-15: Blue to White progression
  15: { core: "#FFFFFF", glow: "#99FFFF", trail: ["#FFFFFF", "#99FFFF", "#66CCFF"] },
  // Levels 16-20: Special effects (White + Purple)
  20: { core: "#FFFFFF", glow: "#CC99FF", trail: ["#FFFFFF", "#CC99FF", "#9933FF"] }
};

const SOUND_EFFECTS = {
  cast: {
    base: 'data:audio/mp3;base64,SUQzBAAAAAAAI1RTU0UAAAAPAAADTGF2ZjU4Ljc2LjEwMAAAAAAAAAAAAAAA//tAwAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAAeAAAiUAAVFRUgICAgKysrNTU1NT9AUEBKS0tLVVZWVmBgYGpra2t1dXV/f3+AiYmJk5SUlJ6enqioqKizs7O9vb3HyMjI0tLS3Nzc3Obm5vDw8PD6+vr///8AAAAATGF2YzU4LjEzAAAAAAAAAAAAAAAAJAYrAAAAAAAAIlCKh8CBAAAAAAA9CP9AyL1JnAAAAAAAAAAAAAAAAA==', // existing base64 sound
    high: 'data:audio/mp3;base64,SUQzBAAAAAAAI1RTU0UAAAAPAAADTGF2ZjU4Ljc2LjEwMAAAAAAAAAAAAAAA//tAwAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAAUAAAXQAAYGBghISEhKioqMzMzMzw8PERERERNTk5OV1dXYGBgYGlpaXJycnJ7e3uDg4ODjIyMlZWVlZ6enqampqavr6+4uLi4wcHBycnJydLS0tvb29vk5OTs7Ozs9fX1/f39AAAAATGF2YzU4LjEzAAAAAAAAAAAAAAAAJASwAAAAAAAAF0Cmk3+lQAAAAAAAAAAAAAAAAAA',  // existing base64 sound
    variations: {
      15: { pitch: 1.2, volume: 0.5 },
      16: { pitch: 1.3, volume: 0.6 },
      17: { pitch: 1.4, volume: 0.7 },
      18: { pitch: 1.5, volume: 0.8 },
      19: { pitch: 1.6, volume: 0.9 },
      20: { pitch: 1.7, volume: 1.0 }
    }
  }
};

const SHADER_VERTEX = `
    attribute vec3 position;
    attribute vec2 uv;
    uniform mat4 modelViewMatrix;
    uniform mat4 projectionMatrix;
    varying vec2 vUv;
    void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
`;

const SHADER_FRAGMENT = `
    precision highp float;
    varying vec2 vUv;
    varying vec3 vNormal;
    varying vec3 vViewPosition;

    uniform sampler2D diffuseMap;
    uniform vec3 color;
    uniform float opacity;
    uniform vec3 lightPosition;

    void main() {
        vec3 normal = normalize(vNormal);
        vec3 lightDir = normalize(lightPosition - vViewPosition);
        float diff = max(dot(normal, lightDir), 0.0);

        vec4 texColor = texture2D(diffuseMap, vUv);
        vec3 diffuse = color * texColor.rgb * diff;

        // Add rim lighting
        float rim = 1.0 - max(dot(normal, normalize(-vViewPosition)), 0.0);
        rim = pow(rim, 3.0);

        gl_FragColor = vec4(diffuse + rim * color, texColor.a * opacity);
    }
`;

// Add WebGL shader for post-processing
const POST_PROCESS_FRAGMENT = `
    precision highp float;
    uniform sampler2D tDiffuse;
    uniform vec2 resolution;
    uniform float time;

    float noise(vec2 p) {
        return fract(sin(dot(p.xy, vec2(12.9898,78.233))) * 43758.5453);
    }

    void main() {
        vec2 uv = gl_FragCoord.xy / resolution;
        vec4 color = texture2D(tDiffuse, uv);

        // Heat distortion
        float distortion = sin(uv.y * 10.0 + time) * 0.001;
        color += texture2D(tDiffuse, uv + vec2(distortion)) * 0.5;

        // Add bloom
        float brightness = dot(color.rgb, vec3(0.299, 0.587, 0.114));
        if(brightness > 0.7) {
            color += color * 2.0;
        }

        gl_FragColor = color;
    }
`;

class ParticleSystem {
    constructor(gl) {
        this.gl = gl;
        this.particles = [];
        this.program = this.createShaderProgram();
        this.geometry = this.createGeometry();
        this.texture = this.createParticleTexture();

        // Particle pooling
        this.particlePool = new Array(1000).fill(null).map(() => ({
            position: new Float32Array(3),
            velocity: new Float32Array(3),
            color: new Float32Array(3),
            age: 0,
            alive: false
        }));
    }

    createShaderProgram() {
        // ... shader compilation code ...
    }

    update(deltaTime) {
        const cullingFrustum = this.calculateFrustum();

        // Multi-threaded particle updates using Web Workers
        if (window.Worker && !this.worker) {
            this.worker = new Worker('particle-worker.js');
            this.worker.onmessage = (e) => this.updateParticleBuffers(e.data);
        }

        // Batch updates
        const updateBatch = this.particles.filter(p =>
            p.alive && this.isInFrustum(p.position, cullingFrustum));

        if (this.worker) {
            this.worker.postMessage({
                particles: updateBatch,
                deltaTime: deltaTime
            });
        } else {
            this.updateParticlesCPU(updateBatch, deltaTime);
        }
    }

    render() {
        // GPU instancing for particles
        const instanceData = new Float32Array(this.particles.length * 16);
        let offset = 0;

        for (const particle of this.particles) {
            if (!particle.alive) continue;
            // Pack instance data
            // ... matrix transformation code ...
            offset += 16;
        }

        // Single draw call for all particles
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.instanceBuffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, instanceData, this.gl.DYNAMIC_DRAW);
        this.gl.drawArraysInstanced(this.gl.TRIANGLES, 0, 6, this.particles.length);
    }
}

// Update sound system for 3D audio
const AUDIO_CONTEXT = new (window.AudioContext || window.webkitAudioContext)();

class SpatialAudioSystem {
    constructor() {
        this.context = AUDIO_CONTEXT;
        this.listener = this.context.listener;

        // Create audio buffer cache
        this.soundCache = new Map();
    }

    async loadSound(url) {
        if (this.soundCache.has(url)) {
            return this.soundCache.get(url);
        }

        const response = await fetch(url);
        const arrayBuffer = await response.arrayBuffer();
        const audioBuffer = await this.context.decodeAudioData(arrayBuffer);
        this.soundCache.set(url, audioBuffer);
        return audioBuffer;
    }

    playSpatialSound(buffer, position, options = {}) {
        const source = this.context.createBufferSource();
        const panner = this.context.createPanner();

        panner.panningModel = 'HRTF';
        panner.distanceModel = 'inverse';
        panner.refDistance = 1;
        panner.maxDistance = 10000;
        panner.rolloffFactor = 1;
        panner.coneInnerAngle = 360;
        panner.coneOuterAngle = 0;
        panner.coneOuterGain = 0;

        panner.setPosition(position.x, position.y, position.z);

        source.buffer = buffer;
        source.connect(panner).connect(this.context.destination);
        source.start(0);

        return { source, panner };
    }
}

function getFireballColors(level) {
  level = Math.min(20, Math.max(1, level));
  const levels = Object.keys(FIREBALL_LEVELS).map(Number).sort((a, b) => a - b);

  let lower = levels[0];
  let upper = levels[0];

  for (const lvl of levels) {
    if (lvl <= level) {
      lower = lvl;
    }
    if (lvl >= level) {
      upper = lvl;
      break;
    }
  }

  const colors = { ...FIREBALL_LEVELS[lower] };
  if (level >= 15) {
    colors.special = true;
    colors.level = level;
  }
  return colors;
}

/**
 * Creates a fireball element
 * @returns {HTMLElement} The fireball DOM element
 */
function createFireballElement(level = 1) {
  const fireball = document.createElement('div');
  fireball.className = 'fireball';
  const colors = getFireballColors(level);

  // Core of the fireball
  const core = document.createElement('div');
  core.className = 'fireball-core';

  // Glow effect around the fireball
  const glow = document.createElement('div');
  glow.className = 'fireball-glow';

  // Append parts to the fireball
  fireball.appendChild(glow);
  fireball.appendChild(core);

  // Add CSS to the element
  fireball.style.cssText = `
    position: absolute;
    width: ${FIREBALL_CONFIG.size.initial}px;
    height: ${FIREBALL_CONFIG.size.initial}px;
    pointer-events: none;
    z-index: 1000;
  `;

  core.style.cssText = `
    width: 60%;
    height: 60%;
    background-color: ${colors.core};
    border-radius: 50%;
    position: absolute;
    top: 20%;
    left: 20%;
    animation: pulse 0.5s infinite alternate;
  `;

  glow.style.cssText = `
    width: 100%;
    height: 100%;
    background: radial-gradient(circle, ${colors.core} 0%, ${colors.glow} 70%, transparent 100%);
    border-radius: 50%;
    position: absolute;
    filter: blur(5px);
    opacity: 0.7;
    animation: glow 0.7s infinite alternate;
  `;

  if (colors.special) {
    const specialEffects = {
      16: () => addDragonHeads(fireball),
      17: () => addVortexEffect(fireball),
      18: () => addMultiCore(fireball),
      19: () => addPhoenixWings(fireball),
      20: () => addTimeDistortion(fireball)
    };

    if (specialEffects[colors.level]) {
      specialEffects[colors.level]();
    }
  }

  return fireball;
}

function addDragonHeads(fireball) {
  const heads = document.createElement('div');
  heads.className = 'dragon-heads';
  heads.style.cssText = `
    position: absolute;
    width: 200%;
    height: 200%;
    animation: rotate 2s infinite linear;
  `;
  fireball.appendChild(heads);
}

// Add other special effect functions here...

/**
 * Creates particle elements for the trail effect
 * @param {HTMLElement} parent - The parent element to append particles to
 */
function createParticles(parent, level) {
  const container = document.createElement('div');
  container.className = 'particle-container';
  container.style.cssText = `
    position: absolute;
    width: 0;
    height: 0;
    pointer-events: none;
    transform-style: preserve-3d;
  `;

  const colors = getFireballColors(level);
  const particleCount = FIREBALL_CONFIG.particles.count * (1 + (level * 0.2));

  for (let i = 0; i < particleCount; i++) {
    const particle = document.createElement('div');
    particle.className = 'fireball-particle';

    const size = FIREBALL_CONFIG.particles.baseSize.min +
                 Math.random() * (FIREBALL_CONFIG.particles.baseSize.max - FIREBALL_CONFIG.particles.baseSize.min);

    const angle = (Math.random() - 0.5) * FIREBALL_CONFIG.particles.spread;
    const speed = FIREBALL_CONFIG.particles.speed.min +
                  Math.random() * (FIREBALL_CONFIG.particles.speed.max - FIREBALL_CONFIG.particles.speed.min);

    const z = (Math.random() - 0.5) * FIREBALL_CONFIG.perspective.depth;

    const colorIndex = Math.floor(Math.random() * colors.trail.length);
    const lifetime = FIREBALL_CONFIG.particles.lifetime * (0.8 + Math.random() * 0.4);

    particle.style.cssText = `
      width: ${size}px;
      height: ${size}px;
      background: radial-gradient(circle, ${colors.trail[colorIndex]} 0%, ${colors.glow} 70%, transparent 100%);
      border-radius: 50%;
      position: absolute;
      filter: blur(2px);
      transform: translate3d(${Math.cos(angle) * speed}px,
                           ${Math.sin(angle) * speed}px,
                           ${z}px);
      opacity: ${FIREBALL_CONFIG.particles.opacity.start};
      animation: particle-fade ${lifetime}ms forwards;
    `;

    container.appendChild(particle);
  }

  parent.appendChild(container);
  return container;
}

function createEnhancedParticles(parent, level) {
  const container = createParticles(parent, level);

  if (level >= 15) {
    const physics = {
      gravity: 0.1 * (level - 14),
      wind: Math.sin(performance.now() / 1000) * 0.5,
      turbulence: 0.2 * (level - 14)
    };

    const particles = container.querySelectorAll('.fireball-particle');
    particles.forEach(particle => {
      particle.physics = physics;
      particle.velocity = {
        x: (Math.random() - 0.5) * 10,
        y: (Math.random() - 0.5) * 10,
        z: (Math.random() - 0.5) * 10
      };
    });
  }

  return container;
}

/**
 * Creates CSS for the animations
 * @returns {HTMLStyleElement} The style element with animation definitions
 */
function createFireballStyles() {
  const style = document.createElement('style');
  style.textContent = `
    @keyframes pulse {
      0% { transform: scale(0.9); }
      100% { transform: scale(1.1); }
    }

    @keyframes glow {
      0% { opacity: 0.5; filter: blur(5px); }
      100% { opacity: 0.8; filter: blur(7px); }
    }

    @keyframes fadeOut {
      0% { opacity: 0.7; }
      100% { opacity: 0; transform: translate(var(--x), var(--y)); }
    }

    @keyframes explode {
      0% { transform: scale(1); opacity: 1; }
      100% { transform: scale(3); opacity: 0; }
    }

    @keyframes particle-fade {
      0% {
        opacity: ${FIREBALL_CONFIG.particles.opacity.start};
        transform: translate3d(0, 0, 0) scale(1);
      }
      100% {
        opacity: ${FIREBALL_CONFIG.particles.opacity.end};
        transform: translate3d(var(--x), var(--y), var(--z)) scale(0.5);
      }
    }

    @keyframes fireball-rotate3d {
      0% { transform: rotate3d(1, 1, 1, 0deg); }
      100% { transform: rotate3d(1, 1, 1, 360deg); }
    }
  `;

  return style;
}

/**
 * Animate the fireball from start to target
 * @param {number} startX - Starting X coordinate
 * @param {number} startY - Starting Y coordinate
 * @param {number} targetX - Target X coordinate
 * @param {number} targetY - Target Y coordinate
 * @param {number} level - Spell level (1-20)
 */
function castFireball(startX, startY, targetX, targetY, level = 1) {
  // Add the animation styles if they don't exist yet
  if (!document.getElementById('fireball-styles')) {
    const styles = createFireballStyles();
    styles.id = 'fireball-styles';
    document.head.appendChild(styles);
  }

  // Create the fireball element
  const fireball = createFireballElement(level);
  document.body.appendChild(fireball);

  // Position at start coordinates
  fireball.style.left = `${startX - FIREBALL_CONFIG.size.initial / 2}px`;
  fireball.style.top = `${startY - FIREBALL_CONFIG.size.initial / 2}px`;

  // Calculate distance and direction
  const dx = targetX - startX;
  const dy = targetY - startY;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // Calculate travel time based on distance and speed
  const travelTime = distance / FIREBALL_CONFIG.speed * 1000; // in ms

  // Play cast sound
  playSoundEffect('cast', level);

  // Track animation start time
  const startTime = performance.now();

  // Create particles while moving
  const particleInterval = setInterval(() => {
    createEnhancedParticles(fireball, level);
  }, 50);

  // Animate the fireball
  function animateFireball(timestamp) {
    const elapsed = timestamp - startTime;
    const progress = Math.min(elapsed / travelTime, 1);

    // Calculate current position
    const currentX = startX + dx * progress;
    const currentY = startY + dy * progress;

    // Update position
    fireball.style.left = `${currentX - FIREBALL_CONFIG.size.initial / 2}px`;
    fireball.style.top = `${currentY - FIREBALL_CONFIG.size.initial / 2}px`;

    // Gradually increase size
    const currentSize = FIREBALL_CONFIG.size.initial + (FIREBALL_CONFIG.size.final - FIREBALL_CONFIG.size.initial) * progress;
    fireball.style.width = `${currentSize}px`;
    fireball.style.height = `${currentSize}px`;

    // Continue animation if not complete
    if (progress < 1) {
      requestAnimationFrame(animateFireball);
    } else {
      // Stop generating particles
      clearInterval(particleInterval);

      // Explosion effect
      fireball.classList.add('exploding');
      fireball.style.animation = 'explode 0.5s forwards';

      // Play impact sound
      const impactSound = new Audio();
      impactSound.src = 'data:audio/mp3;base64,SUQzBAAAAAAAI1RTU0UAAAAPAAADTGF2ZjU4Ljc2LjEwMAAAAAAAAAAAAAAA//tAwAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAAUAAAXQAAYGBghISEhKioqMzMzMzw8PERERERNTk5OV1dXYGBgYGlpaXJycnJ7e3uDg4ODjIyMlZWVlZ6enqampqavr6+4uLi4wcHBycnJydLS0tvb29vk5OTs7Ozs9fX1/f39AAAAATGF2YzU4LjEzAAAAAAAAAAAAAAAAJASwAAAAAAAAF0Cmk3+lQAAAAAAAAAAAAAAAAAA';
      impactSound.volume = 0.5;
      impactSound.play();

      // Remove the fireball after explosion
      setTimeout(() => {
        if (fireball.parentNode) {
          document.body.removeChild(fireball);
        }
      }, 500);
    }
  }

  // Start animation
  requestAnimationFrame(animateFireball);

  // Update particle system
  if (!window.particleSystem) {
    const canvas = document.getElementById('glCanvas');
    const gl = canvas.getContext('webgl2');
    window.particleSystem = new OptimizedParticleSystem(gl);
  }

  // Create optimized particle batch
  const particleCount = Math.min(
    FIREBALL_CONFIG.particles.count * (1 + (level * 0.1)),
    FIREBALL_CONFIG.particles.maxParticles
  );

  // Use object pooling for particles
  const particles = window.particleSystem.getParticlesFromPool(particleCount);

  // Play spatial audio
  if (!window.audioSystem) {
    window.audioSystem = new SpatialAudioSystem();
  }

  const audioPosition = { x: startX, y: startY, z: 0 };
  window.audioSystem.playSpatialSound(
    SOUND_EFFECTS.cast.buffer,
    audioPosition,
    { level: level }
  );
}

function playSoundEffect(type, level) {
  const sound = new Audio();
  const variation = SOUND_EFFECTS[type].variations[level] || { pitch: 1, volume: 0.3 };

  sound.src = level >= 15 ? SOUND_EFFECTS[type].high : SOUND_EFFECTS[type].base;
  sound.playbackRate = variation.pitch;
  sound.volume = variation.volume;

  return sound.play();
}

/**
 * Initializes the fireball spell for use on a webpage
 * @param {string} containerSelector - CSS selector for the container where the fireball can be cast
 * @param {number} level - Spell level (1-20)
 */
export function initFireballSpell(containerSelector = 'body', level = 1) {
  const container = document.querySelector(containerSelector);

  if (!container) {
    console.error(`Container "${containerSelector}" not found`);
    return;
  }

  container.addEventListener('click', (event) => {
    // Cast from bottom center of screen to click position
    const startX = window.innerWidth / 2;
    const startY = window.innerHeight - 50;
    castFireball(startX, startY, event.clientX, event.clientY, level);
  });

  console.log(`Level ${level} Fireball spell initialized. Click anywhere to cast!`);

  // Initialize optimized systems
  const canvas = document.getElementById('glCanvas');
  const gl = canvas.getContext('webgl2', {
      antialias: true,
      powerPreference: 'high-performance'
  });

  window.particleSystem = new OptimizedParticleSystem(gl);

  // Enable WebGL optimizations
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.CULL_FACE);
  gl.enable(gl.BLEND);
}

// Expose the casting function globally
window.castFireball = castFireball;

class FireballRenderer {
    constructor(gl) {
        this.gl = gl;
        this.mesh = new FireballMesh(gl);
        this.audioManager = new SpatialAudioManager();
        this.setupScene();
    }

    setupScene() {
        // Setup WebGL state
        this.gl.enable(this.gl.DEPTH_TEST);
        this.gl.enable(this.gl.CULL_FACE);
        this.gl.enable(this.gl.BLEND);
        this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE_MINUS_SRC_ALPHA);

        // Create camera
        this.camera = new Camera(75, window.innerWidth / window.innerHeight);

        // Initialize post-processing
        this.postProcess = new PostProcessor(this.gl);
    }

    render(fireballs) {
        // Update camera
        this.camera.update();
        this.audioManager.updateListener(this.camera);

        // Clear scene
        this.gl.clear(this.gl.COLOR_BUFFER_BIT | this.gl.DEPTH_BUFFER_BIT);

        // Render fireballs
        this.mesh.render(this.camera, fireballs, window.currentSpellLevel);

        // Apply post-processing
        this.postProcess.render();
    }
}

class FireballRenderer {
    constructor(gl) {
        this.gl = gl;
        this.scene = new Scene();
        this.camera = new PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        this.renderer = new WebGLRenderer({ canvas: gl.canvas, alpha: true });

        this.initPostProcessing();
        this.setupLighting();
    }

    setupLighting() {
        const light = new PointLight(0xff4400, 1, 100);
        this.scene.add(light);

        // Add ambient occlusion
        const aoPass = new SAOPass(this.scene, this.camera);
        this.composer.addPass(aoPass);
    }

    render(particles, fireballPosition) {
        // Update particle positions using Vector3 for better performance
        particles.forEach(particle => {
            particle.position.copy(
                particle.velocity.multiplyScalar(particle.deltaTime)
            );
        });

        // Batch instancing for particles
        this.instancedMesh.count = particles.length;
        this.instancedMesh.instanceMatrix.needsUpdate = true;

        // Update post-processing uniforms
        this.postProcess.uniforms.time.value = performance.now() / 1000;

        // Render with composer for post-processing
        this.composer.render();
    }
}

// Update particle system with better physics
class EnhancedParticleSystem extends ParticleSystem {
    constructor(gl) {
        super(gl);
        this.vectorPool = new VectorPool(1000);
        this.quadtree = new Quadtree({ x: 0, y: 0, width: window.innerWidth, height: window.innerHeight });
    }

    update(deltaTime) {
        // Use quadtree for collision detection
        this.quadtree.clear();
        this.particles.forEach(p => this.quadtree.insert(p));

        // Batch physics updates
        const chunks = this.chunkParticles(this.particles, 100);
        chunks.forEach(chunk => {
            this.updateParticleChunk(chunk, deltaTime);
        });
    }

    updateParticleChunk(particles, dt) {
        particles.forEach(particle => {
            // Use vector pool for calculations
            const velocity = this.vectorPool.get()
                .copy(particle.velocity)
                .add(this.calculateForces(particle, dt));

            particle.position.add(velocity.multiplyScalar(dt));
            this.vectorPool.release(velocity);
        });
    }
}
