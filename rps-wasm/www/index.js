import * as rps_engine from "rps-wasm";

const Choice = {Rock:0, Paper:1, Scissors:2};

const game = rps_engine.Game.new();
function refresh() {
    const board = document.getElementById("board");
    board.innerText = game.make_score_board();
}

function OnClickedButton() {
    if (this.id === "rock_btn") {
        game.play_game(Choice.Rock);
    } else if(this.id === "paper_btn") {
        game.play_game(Choice.Paper);
    } else if(this.id === "scissors_btn") {
        game.play_game(Choice.Scissors);
    }

    refresh()
}

var rock_btn = document.getElementById("rock_btn");
rock_btn.addEventListener('click', OnClickedButton);

var paper_btn = document.getElementById("paper_btn");
paper_btn.addEventListener('click', OnClickedButton);

var scissors_btn = document.getElementById("scissors_btn");
scissors_btn.addEventListener('click', OnClickedButton);