<script lang="ts">
  import Piece from "./Piece.svelte";
  import type {Move} from "./types";
  import {Colour} from "./types";

  export let animal: string;
  export let moves: Move[];

  export let colour: Colour | null = null;
  export let active = false;
  export let selected = false;
</script>

<div class="relative aspect-video rounded-2xl overflow-clip {colour == null ? 'bg-slate-500' : colour === Colour.Red ? 'bg-pink-500' : 'bg-blue-500'} {active && selected ? '-translate-y-4' : ''} transition-transform shadow-2xl flex flex-row items-center">
    <div class="h-full grid grid-cols-5 grid-rows-5 aspect-square">
        {#each {length: 5} as _, row}
            {#each {length: 5} as _, col}
                <div class="relative {row % 2 === col % 2 ? 'bg-slate-200' : 'bg-slate-300'}">
                    {#if row === 2 && col === 2}
                        <Piece colour={colour}/>
                    {:else if moves.some(m => m.dx === col - 2 && m.dy === row - 2)}
                        <div class="w-full h-full {row % 2 === col % 2 ? (colour == null ? 'bg-slate-300' : colour === Colour.Red ? 'bg-pink-300' : 'bg-blue-300') : (colour == null ? 'bg-slate-400' : colour === Colour.Red ? 'bg-pink-400' : 'bg-blue-400')}"></div>
                    {/if}
                </div>
            {/each}
        {/each}
    </div>
    <span class="w-full text-2xl text-center text-slate-200 font-bold pointer-events-none">{animal}</span>
    {#if active}
        <div on:click class="absolute w-full h-full bg-yellow-400 {selected ? 'opacity-20' : 'opacity-0'} hover:opacity-30 transition-opacity cursor-pointer"></div>
    {/if}
</div>
