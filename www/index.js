import * as wasm from "rust-control";

function logspace(start, end, total) {
    const div = end / start;
    let step = Math.log(div) / total;
    step = Math.exp(step);
    let val = start;
    return new Array(total).fill(0).map(
        (_, idx) => {
            if (idx !== 0) val = val * step
            return val
        }
    )
}

function Bode_plot(omegas, syslist, visibles) {
    const data = [];
    for (const idx in syslist) {
        if (!visibles[idx]) {
            continue
        }
        const sys = syslist[idx];

        data.push(
            {
                x: omegas,
                y: sys.sine.map(val => val[2]),
                name: sys.name,
                visible: visibles[idx],
                legendgroud: sys.name,
                marker: sys.marker,
                type: 'scatter'
            },
            {
                x: omegas,
                y: sys.sine.map(val => val[3] / Math.PI * 180),
                name: sys.name,
                visible: visibles[idx],
                legendgroud: sys.name,
                marker: sys.marker,
                showlegend: false,
                xaxis: 'x',
                yaxis: 'y2',
                type: 'scatter'
            }
        )
    };
    const layout = {
        title: "波特图",
        xaxis: {
            type: 'log'
        },
        yaxis: {
            type: 'log',
            anchor: 'y2'
        },

        grid: {
            rows: 2,
            columns: 1,
            subplots: [['xy2'], ['xy']],
            roworder: 'bottom to top'
        }
    };
    Plotly.newPlot('bode_plot', data, layout);
}

function Nyquist_plot(omegas, syslist, visibles) {
    const data = syslist.map(
        (sys, idx) => {
            const data = sys.sine.filter(
                val => val[2] < 5
            )
            return {
                r: data.map(val => val[2]),
                theta: data.map(val => val[3]),
                thetaunit: "radians",
                name: sys.name,
                marker: sys.marker,
                visible: visibles[idx],
                mode: 'lines',
                type: 'scatterpolar'
            }
        }
    )

    const layout = {
        title: "奈奎斯特图",
        raxis: {
            type: 'log'
        }
    }

    Plotly.newPlot('nyquist_plot', data, layout);
}

function load() {

    let T = document.getElementById('T').value;
    T = parseFloat(T);
    let omega_n = document.getElementById('omega_n').value;
    omega_n = parseFloat(omega_n);
    let zeta = document.getElementById('zeta').value;
    zeta = parseFloat(zeta);

    const omegas = logspace(1e-3, 1e3, 10000);

    const syslist = [
        {
            name: "积分环节",
            num: [1],
            den: [1, 0],
            marker: {
                color: 'red'
            },
        },
        {
            name: "惯性环节",
            num: [1],
            den: [T, 1],
            marker: {
                color: 'MediumSeaGreen'
            },
        },
        {
            name: "振荡环节",
            num: [1],
            den: [1 / omega_n ** 2, 2 * zeta / omega_n, 1],
            marker: {
                color: 'Orange'
            },
        },
        {
            name: "微分环节",
            num: [1, 0],
            den: [1],
            marker: {
                color: 'fuchsia'
            },
        },
        {
            name: "一阶微分环节",
            num: [T, 1],
            den: [1],
            marker: {
                color: 'grey'
            },
        },
        {
            name: "二阶微分环节",
            num: [1 / omega_n ** 2, 2 * zeta / omega_n, 1],
            den: [1],
            marker: {
                color: 'Cyan'
            },
        }
    ].map(val => {
        return {
            ...val,
            sine: omegas.map(o => wasm.sine(val.num, val.den, o))
        }
    })

    const visibles = syslist.map((val, idx) => document.getElementById("plot" + (idx + 1).toString()).checked)

    Bode_plot(omegas, syslist, visibles);
    Nyquist_plot(omegas, syslist, visibles);
}

load();

document.getElementById('plot0').addEventListener(
    'change', e => {
        for (let i = 1; i <= 6; i++) {
            const ele = document.getElementById('plot' + i.toString());
            ele.checked = e.target.checked;
        }
        load()
    }
)

for (const ele of document.body.getElementsByClassName("sysvisible")) {
    ele.addEventListener('change', load)
}

for (const ele of document.body.getElementsByClassName("sysparam")) {
    ele.addEventListener('change', load)
}
