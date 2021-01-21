import {TicTacToe} from "tic-tac-toe";

const main = document.getElementById('main');
const p = document.getElementById('result');

let ttt = TicTacToe.new();

for (let i = 0; i < 9; i++) {
  let div = document.createElement('div');
  div.id = i;
  div.className = 'cell';
  
  div.addEventListener('click', (e) => {
    let idNumber = Number(e.target.id);
    if (div.textContent === "") {
      div.textContent = ttt.play(idNumber);
    }
    p.textContent = ttt.change_text();
  })

  main.appendChild(div)
}
