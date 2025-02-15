class CollisionSystem {
    constructor() {
        this.objects = [];
        this.player = null;
        this.terrain = null;
    }

    addPlayer(player) {
        this.player = player;
    }

    addTerrain(terrain) {
        this.terrain = terrain;
    }

    addObject(object) {
        this.objects.push(object);
    }

    update() {
        if (!this.player) return;

        // Check terrain collision
        const playerPos = this.player.camera.position;
        const terrainHeight = this.terrain ?
            this.getTerrainHeight(playerPos.x, playerPos.z) : 0;

        if (playerPos.y < terrainHeight + 1.6) {
            playerPos.y = terrainHeight + 1.6;
        }

        // Check object collisions
        this.objects.forEach(obj => {
            const distance = playerPos.distanceTo(obj.position);
            if (distance < 2) {
                // Simple collision response
                const direction = playerPos.clone().sub(obj.position).normalize();
                playerPos.add(direction.multiplyScalar(2 - distance));
            }
        });
    }

    getTerrainHeight(x, z) {
        return this.terrain ?
            this.terrain.geometry.getHeightAt(x, z) : 0;
    }
}
