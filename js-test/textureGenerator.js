const canvas = document.createElement('canvas');
const ctx = canvas.getContext('2d');
canvas.width = 512;
canvas.height = 512;

function generateGrassTexture() {
    // Base green color
    ctx.fillStyle = '#2d5a27';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Add noise for grass detail
    for (let i = 0; i < 5000; i++) {
        ctx.fillStyle = `rgba(${45 + Math.random() * 20}, ${90 + Math.random() * 20}, ${39 + Math.random() * 20}, 0.5)`;
        ctx.fillRect(
            Math.random() * canvas.width,
            Math.random() * canvas.height,
            2,
            10
        );
    }

    return canvas.toDataURL('image/jpeg');
}

function generateRockTexture() {
    // Base gray color
    ctx.fillStyle = '#4a4a4a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Add noise for rock texture
    for (let i = 0; i < 3000; i++) {
        ctx.fillStyle = `rgba(${74 + Math.random() * 30}, ${74 + Math.random() * 30}, ${74 + Math.random() * 30}, 0.7)`;
        ctx.beginPath();
        ctx.arc(
            Math.random() * canvas.width,
            Math.random() * canvas.height,
            Math.random() * 5,
            0,
            Math.PI * 2
        );
        ctx.fill();
    }

    return canvas.toDataURL('image/jpeg');
}

function generateDirtTexture() {
    // Base brown color
    ctx.fillStyle = '#8B4513';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Add noise for dirt texture
    for (let i = 0; i < 4000; i++) {
        ctx.fillStyle = `rgba(${139 + Math.random() * 20}, ${69 + Math.random() * 20}, ${19 + Math.random() * 20}, 0.6)`;
        ctx.beginPath();
        ctx.arc(
            Math.random() * canvas.width,
            Math.random() * canvas.height,
            Math.random() * 3,
            0,
            Math.PI * 2
        );
        ctx.fill();
    }

    return canvas.toDataURL('image/jpeg');
}

// Export textures
const grass = generateGrassTexture();
const rock = generateRockTexture();
const dirt = generateDirtTexture();

// Save textures (you'll need to implement actual file saving in your environment)
