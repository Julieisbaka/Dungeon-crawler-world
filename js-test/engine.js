class GameEngine {
    constructor() {
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        this.renderer = new THREE.WebGLRenderer();
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(this.renderer.domElement);

        this.gameState = {
            health: 100,
            maxHealth: 100,
            stamina: 100,
            maxStamina: 100
        };

        this.ui = new GameUI(this.gameState);
        this.collisionSystem = new CollisionSystem();

        // Load textures
        this.textureLoader = new THREE.TextureLoader();
        this.textures = {
            grass: this.textureLoader.load('assets/textures/grass.jpg'),
            rock: this.textureLoader.load('assets/textures/rock.jpg'),
            dirt: this.textureLoader.load('assets/textures/dirt.jpg')
        };

        this.init();
    }

    init() {
        this.camera.position.y = 1.6; // Player height
        this.camera.position.z = 5;

        // Add basic lighting
        const light = new THREE.AmbientLight(0x404040);
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
        this.scene.add(light);
        this.scene.add(directionalLight);

        // Initialize world and player
        this.worldGenerator = new WorldGenerator(this.scene, this.textures);
        this.player = new Player(this.camera, this.gameState);

        // Setup collision detection
        this.collisionSystem.addTerrain(this.worldGenerator.terrain);
        this.collisionSystem.addPlayer(this.player);

        this.animate();
    }

    animate() {
        requestAnimationFrame(() => this.animate());

        const delta = this.clock.getDelta();
        this.player.update(delta);
        this.collisionSystem.update();
        this.ui.update(this.gameState);

        this.renderer.render(this.scene, this.camera);
    }
}

window.onload = () => {
    const game = new GameEngine();
};
