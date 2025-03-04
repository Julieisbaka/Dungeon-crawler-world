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
  }
};

/**
 * Creates a fireball element
 * @returns {HTMLElement} The fireball DOM element
 */
function createFireballElement() {
  const fireball = document.createElement('div');
  fireball.className = 'fireball';

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
    background-color: ${FIREBALL_CONFIG.colors.core};
    border-radius: 50%;
    position: absolute;
    top: 20%;
    left: 20%;
    animation: pulse 0.5s infinite alternate;
  `;

  glow.style.cssText = `
    width: 100%;
    height: 100%;
    background: radial-gradient(circle, ${FIREBALL_CONFIG.colors.core} 0%, ${FIREBALL_CONFIG.colors.glow} 70%, transparent 100%);
    border-radius: 50%;
    position: absolute;
    filter: blur(5px);
    opacity: 0.7;
    animation: glow 0.7s infinite alternate;
  `;

  return fireball;
}

/**
 * Creates particle elements for the trail effect
 * @param {HTMLElement} parent - The parent element to append particles to
 */
function createParticles(parent) {
  const container = document.createElement('div');
  container.className = 'particle-container';
  container.style.cssText = `
    position: absolute;
    width: 0;
    height: 0;
    pointer-events: none;
  `;

  // Create 8 particles
  for (let i = 0; i < 8; i++) {
    const particle = document.createElement('div');
    particle.className = 'fireball-particle';

    const size = 4 + Math.random() * 8; // Random size between 4-12px
    const angle = Math.random() * Math.PI * 2; // Random angle
    const distance = (FIREBALL_CONFIG.size.initial / 2) * Math.random();
    const colorIndex = Math.floor(Math.random() * FIREBALL_CONFIG.colors.trail.length);

    particle.style.cssText = `
      width: ${size}px;
      height: ${size}px;
      background-color: ${FIREBALL_CONFIG.colors.trail[colorIndex]};
      border-radius: 50%;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(${Math.cos(angle) * distance}px, ${Math.sin(angle) * distance}px);
      opacity: ${0.4 + Math.random() * 0.6};
      animation: fadeOut ${0.2 + Math.random() * 0.4}s forwards;
    `;

    container.appendChild(particle);
  }

  parent.appendChild(container);

  // Remove particles after they fade
  setTimeout(() => {
    if (container.parentNode === parent) {
      parent.removeChild(container);
    }
  }, 500);
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
  `;

  return style;
}

/**
 * Animate the fireball from start to target
 * @param {number} startX - Starting X coordinate
 * @param {number} startY - Starting Y coordinate
 * @param {number} targetX - Target X coordinate
 * @param {number} targetY - Target Y coordinate
 */
function castFireball(startX, startY, targetX, targetY) {
  // Add the animation styles if they don't exist yet
  if (!document.getElementById('fireball-styles')) {
    const styles = createFireballStyles();
    styles.id = 'fireball-styles';
    document.head.appendChild(styles);
  }

  // Create the fireball element
  const fireball = createFireballElement();
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
  const castSound = new Audio();
  castSound.src = 'data:audio/mp3;base64,SUQzBAAAAAAAI1RTU0UAAAAPAAADTGF2ZjU4Ljc2LjEwMAAAAAAAAAAAAAAA//tAwAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAAeAAAiUAAVFRUgICAgKysrNTU1NT9AUEBKS0tLVVZWVmBgYGpra2t1dXV/f3+AiYmJk5SUlJ6enqioqKizs7O9vb3HyMjI0tLS3Nzc3Obm5vDw8PD6+vr///8AAAAATGF2YzU4LjEzAAAAAAAAAAAAAAAAJAYrAAAAAAAAIlCKh8CBAAAAAAA9CP9AyL1JnAAAAAAAAAAAAAAAAA==';
  castSound.volume = 0.3;
  castSound.play();

  // Track animation start time
  const startTime = performance.now();

  // Create particles while moving
  const particleInterval = setInterval(() => {
    createParticles(fireball);
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
}

/**
 * Initializes the fireball spell for use on a webpage
 * @param {string} containerSelector - CSS selector for the container where the fireball can be cast
 */
export function initFireballSpell(containerSelector = 'body') {
  const container = document.querySelector(containerSelector);

  if (!container) {
    console.error(`Container "${containerSelector}" not found`);
    return;
  }

  container.addEventListener('click', (event) => {
    // Cast from bottom center of screen to click position
    const startX = window.innerWidth / 2;
    const startY = window.innerHeight - 50;
    castFireball(startX, startY, event.clientX, event.clientY);
  });

  console.log('Level 1 Fireball spell initialized. Click anywhere to cast!');
}

// Expose the casting function globally
window.castFireball = castFireball;
