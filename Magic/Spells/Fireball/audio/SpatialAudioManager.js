class SpatialAudioManager {
    constructor() {
        this.context = new (window.AudioContext || window.webkitAudioContext)();
        this.listener = this.context.listener;
        this.sounds = new Map();
        this.convolver = this.context.createConvolver();
        this.loadImpulseResponse();
    }

    async loadImpulseResponse() {
        const response = await fetch('audio/fire_impulse.wav');
        const arrayBuffer = await response.arrayBuffer();
        this.convolver.buffer = await this.context.decodeAudioData(arrayBuffer);
    }

    createSource(position, options = {}) {
        const source = this.context.createBufferSource();
        const panner = this.context.createPanner();

        // Configure panner
        panner.panningModel = 'HRTF';
        panner.distanceModel = 'inverse';
        panner.refDistance = 1;
        panner.maxDistance = 100;
        panner.rolloffFactor = 1;

        // Position the sound
        panner.setPosition(position.x, position.y, position.z);

        // Setup audio processing chain
        const gainNode = this.context.createGain();
        source.connect(gainNode)
              .connect(panner)
              .connect(this.convolver)
              .connect(this.context.destination);

        return { source, panner, gainNode };
    }

    updateListener(camera) {
        const pos = camera.position;
        const forward = camera.getForward();
        const up = camera.getUp();

        this.listener.setPosition(pos.x, pos.y, pos.z);
        this.listener.setOrientation(forward.x, forward.y, forward.z, up.x, up.y, up.z);
    }
}
