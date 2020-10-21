export class Snow {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    updateBound: () => void;
    snowflakes: Snowflake[];
    width: number = 0;
    height: number = 0;
    constructor() {
        this.snowflakes = []
        this.canvas = document.createElement('canvas');
        this.ctx = this.canvas.getContext('2d')!;
        document.body.appendChild(this.canvas);

        window.addEventListener('resize', () => this.onResize());

        this.updateBound = this.update.bind(this);
        requestAnimationFrame(this.updateBound);
        this.createSnowflakes();
    }
    createSnowflakes() {
        const flakes = window.innerWidth / 4;
        this.snowflakes = []
        for (let s = 0; s < flakes; s++) {
            this.snowflakes.push(new Snowflake());
        }
    }

    update() {
        this.ctx.clearRect(0, 0, this.width, this.height);
        for (const flake of this.snowflakes) {
            flake.update();
            this.ctx.save();
            this.ctx.fillStyle = '#FFF';
            this.ctx.beginPath();
            this.ctx.arc(flake.x, flake.y, flake.radius, 0, Math.PI * 2);
            this.ctx.closePath();
            this.ctx.globalAlpha = flake.alpha;
            this.ctx.fill();
            this.ctx.restore();
        }
        requestAnimationFrame(this.updateBound);
    }

    onResize() {
        this.width = window.innerWidth;
        this.height = window.innerHeight;
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }
}

export class Snowflake {
    x: number = 0;
    y: number = 0;
    vx: number = 0;
    vy: number = 0;
    radius: number = 0;
    alpha: number = 0;
    constructor() {
        this.reset();
    }

    randBetween(min: number, max: number) {
        return min + Math.random() * (max - min);
    }

    update() {
        this.x += this.vx;
        this.y += this.vy;
        if (this.y + this.radius > window.innerHeight) {
            this.reset();
        }
    }

    reset() {
        this.x = this.randBetween(0, window.innerWidth);
        this.y = this.randBetween(0, -window.innerHeight);
        this.vx = this.randBetween(-3, 3);
        this.vy = this.randBetween(2, 5);
        this.radius = this.randBetween(1, 4);
        this.alpha = this.randBetween(0.1, 0.9);
    }
}