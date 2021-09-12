import {Throttle} from 'lander';

async function main() {
    const lander = await import('lander');
    const canvas = document.getElementById('canvas') as HTMLCanvasElement;
    const world = new lander.World(canvas);
    console.log("world ready");
    function render() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;

        world.redraw();
        requestAnimationFrame(render)
    }
    requestAnimationFrame(render);

    function getThrottle(ev: KeyboardEvent | TouchEvent): Throttle | undefined {
        if (ev instanceof KeyboardEvent) {
            if (ev.key == 'ArrowUp') {
                return lander.Throttle.Bottom;
            } else if (ev.key == 'ArrowLeft') {
                return lander.Throttle.Left;
            } else if (ev.key == 'ArrowRight') {
                return lander.Throttle.Right;
            } else {
                return undefined;
            }
        } else if (ev instanceof TouchEvent) {
            if (ev.changedTouches.length != 1) {
                return undefined;
            }

            const x = ev.changedTouches[0].pageX / window.innerWidth;
            const y = ev.changedTouches[0].pageY / window.innerHeight;
            console.log({x, y})
            if (y < 0.3) {
                return lander.Throttle.Bottom;
            }
            if (x < 0.5) {
                return lander.Throttle.Left;
            } else {
                return lander.Throttle.Right;
            }
        } else {
            return undefined;
        }
    }

    function control(ev: KeyboardEvent | TouchEvent, down: boolean) {
        let key = getThrottle(ev);
        if (key !== undefined) {
            world.control(key, down);
        }
    }

    canvas.addEventListener('touchend', (ev) => {
        control(ev, false);
    })

    canvas.addEventListener('touchstart', (ev) => {
        control(ev, true);
    })

    document.body.addEventListener('keydown', (ev) => {
        control(ev, true);
    })

    document.body.addEventListener('keyup', (ev) => {
        control(ev, false);
    })
}

window.addEventListener("load", () => {
    main().then(() => {
    }, (e) => {
        console.error(e);
    })
})