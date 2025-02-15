class GameUI {
    constructor(gameState) {
        this.createUI();
    }

    createUI() {
        const ui = document.createElement('div');
        ui.style.position = 'fixed';
        ui.style.padding = '20px';
        ui.style.color = 'white';

        this.healthBar = this.createBar('health');
        this.staminaBar = this.createBar('stamina');

        ui.appendChild(this.healthBar);
        ui.appendChild(this.staminaBar);
        document.body.appendChild(ui);
    }

    createBar(type) {
        const bar = document.createElement('div');
        bar.style.width = '200px';
        bar.style.height = '20px';
        bar.style.background = '#333';
        bar.style.marginBottom = '10px';

        const fill = document.createElement('div');
        fill.style.width = '100%';
        fill.style.height = '100%';
        fill.style.background = type === 'health' ? '#ff0000' : '#00ff00';

        bar.appendChild(fill);
        return bar;
    }

    update(gameState) {
        this.healthBar.children[0].style.width =
            `${(gameState.health / gameState.maxHealth) * 100}%`;
        this.staminaBar.children[0].style.width =
            `${(gameState.stamina / gameState.maxStamina) * 100}%`;
    }
}
