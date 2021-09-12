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

    function getThrottle(ev: KeyboardEvent | TouchEvent, cb: (t: Throttle) => void) {
        if (ev instanceof KeyboardEvent) {
            if (ev.key == 'ArrowUp') {
                cb(lander.Throttle.Bottom);
            } else if (ev.key == 'ArrowLeft') {
                cb(lander.Throttle.Left);
            } else if (ev.key == 'ArrowRight') {
                cb(lander.Throttle.Right);
            }
        } else if (ev instanceof TouchEvent) {
            for (const touch of ev.changedTouches) {
                const x = touch.pageX / window.innerWidth;
                const y = touch.pageY / window.innerHeight;
                if (y < 0.3) {
                    cb(lander.Throttle.Bottom);
                } else {
                    if (x < 0.5) {
                        cb(lander.Throttle.Left);
                    } else {
                        cb(lander.Throttle.Right);
                    }
                }
            }
        }
    }

    function control(ev: KeyboardEvent | TouchEvent, down: boolean) {
        getThrottle(ev, (key) => {
            world.control(key, down);
        });
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