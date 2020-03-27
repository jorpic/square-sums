import { h, render, Component, createRef } from "preact";
import data from "./data";

console.log(data);
const N = 1000;
const squares = new Set();
for(let i = 0; i < 2*N; ++i) {
  squares.add(i*i);
}

function edges(n) {
  const res = [];
  for(let x = 1; x <= n; ++x) {
    for(let q of squares) {
      const y = q - x;
      if(y > n) break;
      if(y > 0 && y != x) res.push([x-1, y-1]);
    }
  }
  return res;
}

function points(grid, xs) {
  let points = "";
  for(let i = 0; i < xs.length-1; ++i) {
    let x = xs[i]-1;
    let y = xs[i+1]-1;
    points += i%2 == 0
      ? `${x*grid + grid/2} ${y*grid + grid/2} `
      : `${y*grid + grid/2} ${x*grid + grid/2} `;
  }
  return points;
}


class App extends Component {
  constructor() {
    super();
    this.setState({
      time: 0
    });

    setInterval(() => this.setState({time: this.state.time+1}), 100);
  }

  render() {
    const grid = 20;
    const [n, xs] = data.data[this.state.time % data.data.length];
    const polyline = <polyline points={points(grid, xs)}
      stroke="gold"
      fill="none"
      stroke-width="4"/>;

    const pixels = edges(n).map(([x, y]) =>
      <rect
        class="pixel"
        x={x*grid + 0.5}
        y={y*grid + 0.5}
        width={grid-1}
        height={grid-1}
        fill="skyblue"
      />
    );

    const rect = (size, width) =>
      <rect width={size} height={size} fill="none" stroke="lightgray" stroke-width={width}/>;
    const gridPattern =
      <defs>
        <pattern id="smallGrid" width={grid} height={grid} patternUnits="userSpaceOnUse">
          {rect(grid, 0.5)}
        </pattern>
        <pattern id="grid" width={grid*5} height={grid*5} patternUnits="userSpaceOnUse">
          <rect width={grid*5} height={grid*5} fill="url(#smallGrid)"/>
          {rect(grid*5, 1)}
        </pattern>
      </defs>;

    return (
      <div class="container">
        <div class="controls">
          <h1>n = {n}</h1>
          <h2>{this.state.time}</h2>
        </div>
        <div class="gird">
          <svg width="100%" height="100%">
            {gridPattern}
            <rect width="100%" height="100%" fill="url(#grid)" />
            {pixels}
            {polyline}
          </svg>
        </div>
      </div>);
  }
}

render(<App />, document.body);
