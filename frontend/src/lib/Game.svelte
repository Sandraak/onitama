<script lang="ts">
  import Board from "./Board.svelte";
  import Card from "./Card.svelte";
  import type {Board as BoardType, Card as CardType} from "./types";
  import {Colour} from "./types";

  export let player: Colour;

  export let board: BoardType;
  export let currentPlayer: Colour;
  export let spareCard: CardType;
  export let cards: Map<Colour, CardType[]>;

  let selectedCard: number | null = null;

  function selectCard(index: number) {
    if (selectedCard === index) {
      selectedCard = null;
    } else {
      selectedCard = index;
    }
  }
</script>

<div class="h-full p-10 flex flex-row">
    <div class="basis-1/4 p-10 flex flex-col justify-between">
        <div class="flex flex-col gap-10 rotate-180">
            {#each cards.get(Colour.opposite(player)) as card}
                <Card {...card} colour={Colour.opposite(player)}/>
            {/each}
        </div>
        <Card {...spareCard}/>
    </div>
    <div class="basis-1/2 p-10">
        <div class="h-full mx-auto aspect-square">
            <Board on:move {player} {board} {cards} bind:selectedCard/>
        </div>
    </div>
    <div class="basis-1/4 p-10 flex flex-col justify-end gap-10">
        {#each cards.get(player) as card, index}
            <Card on:click={() => selectCard(index)} {...card} colour={player} selected={selectedCard === index} active={player === currentPlayer}/>
        {/each}
    </div>
</div>
