<script lang="ts">
  import Game from "../lib/Game.svelte";
  import type {Board, Card, State} from "../lib/types";
  import {Colour, Rank} from "../lib/types";
  import {page} from "$app/stores";

  const player: Colour = Colour.Red;

  const board: Board = {
    board: [
      [[Rank.Pawn, Colour.Blue], [Rank.Pawn, Colour.Blue], [Rank.Master, Colour.Blue], [Rank.Pawn, Colour.Blue], [Rank.Pawn, Colour.Blue]],
      [null, null, null, null, null],
      [null, null, null, null, null],
      [null, null, null, null, null],
      [[Rank.Pawn, Colour.Red], [Rank.Pawn, Colour.Red], [Rank.Master, Colour.Red], [Rank.Pawn, Colour.Red], [Rank.Pawn, Colour.Red]],
    ]
  };
  const currentPlayer: Colour = Colour.Red;
  const spareCard: Card = {animal: 'Turtle', moves: [{dx: 0, dy: -1}]};
  const cards: Map<Colour, Card[]> = new Map([
    [Colour.Red, [{animal: 'Tiger', moves: [{dx: 0, dy: -1}]}, {animal: 'Panther', moves: [{dx: 0, dy: -1}]}]],
    [Colour.Blue, [{animal: 'Lion', moves: [{dx: 0, dy: -1}]}, {animal: 'Cheetah', moves: [{dx: 0, dy: -1}]}]],
  ]);
  const state: State = {
    board: board,
    currentPlayer: currentPlayer,
    spareCard: spareCard,
    cards: cards
  };

  async function handleMove(event) {
    const game = await fetch(`/api/submit/${$page.url.searchParams.get('game_id')}/${JSON.stringify(event.detail)}`);
    const response = await game.text();
    alert(response);
    if(response === "Prima.") {
      alert("Prima!");
    } else {
      alert(":(");
    }
  }
</script>

<div class="w-screen h-screen bg-slate-800">
    <Game on:move={handleMove} {player} {...state}/>
</div>
