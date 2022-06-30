<script lang="ts">
  import Card from "./Card.svelte";
  import Piece from "./Piece.svelte";
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  type Move = {
    dx: number;
    dy: number;
  };

  type Card = {
    animal: string;
    moves: Array<Move>;
  };

  enum Rank {
    Pawn = "PAWN",
    Master = "MASTER",
  }

  enum Colour {
    Red = "RED",
    Blue = "BLUE",
  }

  type Position = {
    x: number;
    y: number;
  };

  type MovePiece = {
    from: Position;
    mov: Move;
    card: number;
  }

  type Piece = [Rank, Colour];

  export let player: Colour;

  export let board: (Piece | null)[][] = [
    [[Rank.Pawn, Colour.Blue], null, null, null, [Rank.Pawn, Colour.Red]],
    [[Rank.Pawn, Colour.Blue], null, null, null, [Rank.Pawn, Colour.Red]],
    [[Rank.Master, Colour.Blue], null, null, null, [Rank.Master, Colour.Red]],
    [[Rank.Pawn, Colour.Blue], null, null, null, [Rank.Pawn, Colour.Red]],
    [[Rank.Pawn, Colour.Blue], null, null, null, [Rank.Pawn, Colour.Red]],
  ];
  export let current_player: Colour = Colour.Red;
  export let spare_card: Card = {animal: "Turtle", moves: []};
  export let cards: Map<Colour, Card[]> = new Map([
    [Colour.Red, [{animal: "Turtle", moves: []}, {animal: "Turtle", moves: []}]],
    [Colour.Blue, [{animal: "Turtle", moves: []}, {animal: "Turtle", moves: []}]]
  ]);

  let selected_card: number | null = null;
  let selected_pos: Position | null = null;
  let selected_mov: Move | null = null;

  function selectCard(colour: Colour, index: number) {
    selected_card = index;
  }

  function selectTile(x: number, y: number) {
    if (selected_pos == null) {
      selected_pos = { x, y };
    } else {
      const dx = x - selected_pos.x;
      const dy = y - selected_pos.y;
      selected_mov = { dx, dy };
      const perform = {
        from: selected_pos,
        mov: selected_mov,
        card: selected_card,
      };
      dispatch('move', {text: JSON.stringify(perform)});
      selected_card = null;
      selected_pos = null;
      selected_mov = null;
    }
  }
</script>

<div class="bg-slate-800">
    <div class="p-6 flex flex-row justify-between">
        <div on:click={() => selectCard(Colour.Blue, 0)}>
            <Card {...cards.get(Colour.Blue)[0]}/>
        </div>
        <div on:click={() => selectCard(Colour.Blue, 1)}>
            <Card {...cards.get(Colour.Blue)[1]}/>
        </div>
    </div>
    <div class="flex flex-row justify-center items-center">
        <div class="bg-slate-600">
            <table class="border-8 border-slate-400">
                <tr>
                    <td on:click={() => selectTile(0, 0)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[0][0] != null}<Piece rank={board[0][0][0]} colour={board[0][0][1]}/>{/if}</td>
                    <td on:click={() => selectTile(1, 0)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[1][0] != null}<Piece rank={board[1][0][0]} colour={board[1][0][1]}/>{/if}</td>
                    <td on:click={() => selectTile(2, 0)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[2][0] != null}<Piece rank={board[2][0][0]} colour={board[2][0][1]}/>{/if}</td>
                    <td on:click={() => selectTile(3, 0)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[3][0] != null}<Piece rank={board[3][0][0]} colour={board[3][0][1]}/>{/if}</td>
                    <td on:click={() => selectTile(4, 0)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[4][0] != null}<Piece rank={board[4][0][0]} colour={board[4][0][1]}/>{/if}</td>
                </tr>
                <tr>
                    <td on:click={() => selectTile(0, 1)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[0][1] != null}<Piece rank={board[0][1][0]} colour={board[0][1][1]}/>{/if}</td>
                    <td on:click={() => selectTile(1, 1)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[1][1] != null}<Piece rank={board[1][1][0]} colour={board[1][1][1]}/>{/if}</td>
                    <td on:click={() => selectTile(2, 1)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[2][1] != null}<Piece rank={board[2][1][0]} colour={board[2][1][1]}/>{/if}</td>
                    <td on:click={() => selectTile(3, 1)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[3][1] != null}<Piece rank={board[3][1][0]} colour={board[3][1][1]}/>{/if}</td>
                    <td on:click={() => selectTile(4, 1)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[4][1] != null}<Piece rank={board[4][1][0]} colour={board[4][1][1]}/>{/if}</td>
                </tr>
                <tr>
                    <td on:click={() => selectTile(0, 2)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[0][2] != null}<Piece rank={board[0][2][0]} colour={board[0][2][1]}/>{/if}</td>
                    <td on:click={() => selectTile(1, 2)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[1][2] != null}<Piece rank={board[1][2][0]} colour={board[1][2][1]}/>{/if}</td>
                    <td on:click={() => selectTile(2, 2)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[2][2] != null}<Piece rank={board[2][2][0]} colour={board[2][2][1]}/>{/if}</td>
                    <td on:click={() => selectTile(3, 2)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[3][2] != null}<Piece rank={board[3][2][0]} colour={board[3][2][1]}/>{/if}</td>
                    <td on:click={() => selectTile(4, 2)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[4][2] != null}<Piece rank={board[4][2][0]} colour={board[4][2][1]}/>{/if}</td>
                </tr>
                <tr>
                    <td on:click={() => selectTile(0, 3)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[0][3] != null}<Piece rank={board[0][3][0]} colour={board[0][3][1]}/>{/if}</td>
                    <td on:click={() => selectTile(1, 3)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[1][3] != null}<Piece rank={board[1][3][0]} colour={board[1][3][1]}/>{/if}</td>
                    <td on:click={() => selectTile(2, 3)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[2][3] != null}<Piece rank={board[2][3][0]} colour={board[2][3][1]}/>{/if}</td>
                    <td on:click={() => selectTile(3, 3)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[3][3] != null}<Piece rank={board[3][3][0]} colour={board[3][3][1]}/>{/if}</td>
                    <td on:click={() => selectTile(4, 3)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[4][3] != null}<Piece rank={board[4][3][0]} colour={board[4][3][1]}/>{/if}</td>
                </tr>
                <tr>
                    <td on:click={() => selectTile(0, 4)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[0][4] != null}<Piece rank={board[0][4][0]} colour={board[0][4][1]}/>{/if}</td>
                    <td on:click={() => selectTile(1, 4)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[1][4] != null}<Piece rank={board[1][4][0]} colour={board[1][4][1]}/>{/if}</td>
                    <td on:click={() => selectTile(2, 4)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[2][4] != null}<Piece rank={board[2][4][0]} colour={board[2][4][1]}/>{/if}</td>
                    <td on:click={() => selectTile(3, 4)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[3][4] != null}<Piece rank={board[3][4][0]} colour={board[3][4][1]}/>{/if}</td>
                    <td on:click={() => selectTile(4, 4)} class="w-24 h-24 border-4 border-slate-400 hover:bg-slate-500">{#if board[4][4] != null}<Piece rank={board[4][4][0]} colour={board[4][4][1]}/>{/if}</td>
                </tr>
            </table>
        </div>
        <div class="p-10">
            <Card {...spare_card}/>
        </div>
    </div>
    <div class="p-6 flex flex-row justify-between">
        <div on:click={() => selectCard(Colour.Red, 0)}>
            <Card  {...cards.get(Colour.Red)[0]}/>
        </div>
        <div on:click={() => selectCard(Colour.Red, 1)}>
            <Card {...cards.get(Colour.Red)[1]}/>
        </div>
    </div>
</div>
