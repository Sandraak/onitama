<script lang="ts">
  import {Colour, Rank} from "./types";

  export let rank: Rank = Rank.Pawn;
  export let colour: Colour | null = null;
</script>

<div class="absolute w-full h-full flex flex-col items-center justify-center">
    {#if rank === Rank.Master}
        <div class="w-1/3 h-1/3 {colour == null ? 'bg-slate-500' : colour === Colour.Red ? 'bg-pink-500' : 'bg-blue-500'} rounded-b-full"></div>
        <div class="w-1/2 h-1/2 {colour == null ? 'bg-slate-500' : colour === Colour.Red ? 'bg-pink-500' : 'bg-blue-500'} rounded-t-full"></div>
    {:else}
        <div class="w-1/3 h-1/6"></div>
        <div class="w-1/3 h-1/3 {colour == null ? 'bg-slate-500' : colour === Colour.Red ? 'bg-pink-500' : 'bg-blue-500'} rounded-full"></div>
        <div class="w-1/2 h-1/3 {colour == null ? 'bg-slate-500' : colour === Colour.Red ? 'bg-pink-500' : 'bg-blue-500'} rounded-t-full"></div>
    {/if}
</div>
