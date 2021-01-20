import * as wasm from "tic-tac-toe";

const main = document.getElementById('main');
const p = document.getElementById('result');

for (let i = 0; i < 9; i++) {
  let div = document.createElement('div');
  div.id = i;
  div.className = 'cell';
  div.addEventListener('click', (e) => {
    let idNumber = Number(e.target.id)
    wasm.get_id(idNumber);
    //p.textContent =
  })
  main.appendChild(div)
}


