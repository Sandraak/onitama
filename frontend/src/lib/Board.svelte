<script lang="ts">
  import Piece from "./Piece.svelte";
  import type {Board, Card as CardType, MovePiece, Position} from "./types";
  import {Colour} from "./types";
  import {createEventDispatcher} from "svelte";

  export let player: Colour;

  export let board: Board;
  export let cards: Map<Colour, CardType[]>;

  export let selectedCard: number | null = null;

  let selectedPiece: Position | null = null;
  let card: CardType | null;
  $: {
    selectedPiece = null;
    card = cards?.get(player)[selectedCard];
  }

  const dispatch = createEventDispatcher();

  function selectPiece(row: number, col: number) {
    if(selectedPiece != null && selectedPiece.x === col && selectedPiece.y === row) {
      selectedPiece = null;
    } else {
      selectedPiece = {
        x: col,
        y: row
      };
    }
  }

  function selectTile(row: number, col: number) {
    const move: MovePiece = {
      card: selectedCard,
      from: selectedPiece,
      mov: {dx: col - selectedPiece.x, dy: row - selectedPiece.y}
    }
    dispatch('move', move);
    selectedCard = null;
    selectedPiece = null;
  }
</script>

<div class="w-full h-full rounded-2xl overflow-clip shadow-2xl grid grid-cols-5 grid-rows-5">
    {#each player === Colour.Blue ? [4, 3, 2, 1, 0] : [0, 1, 2, 3, 4] as row}
        {#each {length: 5} as _, col}
            <div class="relative {row % 2 === col % 2 ? 'bg-slate-200' : 'bg-slate-300'}">
                {#if board.board[row][col] != null}
                    <Piece rank={board.board[row][col][0]} colour={board.board[row][col][1]}/>
                {/if}
                {#if card != null && board.board[row][col] != null && board.board[row][col][1] === player}
                    <div on:click={() => selectPiece(row, col)}
                         class="absolute w-full h-full bg-yellow-400 {selectedPiece != null && selectedPiece.x === col && selectedPiece.y === row ? 'opacity-20' : 'opacity-0'} hover:opacity-30 transition-opacity cursor-pointer"></div>
                {/if}
                {#if selectedPiece != null && card != null && card.moves.some(m => selectedPiece.x + m.dx === col && selectedPiece.y + m.dy === row)}
                    <div on:click={() => selectTile(row, col)}
                         class="absolute w-full h-full bg-yellow-400 opacity-20 hover:opacity-30 transition-opacity cursor-pointer"></div>
                {/if}
            </div>
        {/each}
    {/each}
</div>
