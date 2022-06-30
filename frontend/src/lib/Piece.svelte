<script lang="ts">
  import Colour from "./Board.svelte";
  import Rank from "./Board.svelte";

  export let colour: Colour;
  export let rank: Rank;
</script>

{#if rank === "MASTER"}
    {#if colour === "RED"}
        <div class="flex flex-col justify-center">
            <div class="w-6 h-6 bg-pink-500"></div>
            <div class="w-12 h-8 bg-pink-500"></div>
        </div>
    {:else}
        <div class="flex flex-col justify-center">
            <div class="w-6 h-6 bg-blue-500"></div>
            <div class="w-12 h-8 bg-blue-500"></div>
        </div>
    {/if}
{:else}
    {#if colour === "RED"}
        <div class="flex flex-col justify-center">
            <div class="w-6 h-6 bg-pink-500 rounded-full"></div>
            <div class="w-12 h-8 bg-pink-500 rounded-t-full"></div>
        </div>
    {:else}
        <div class="flex flex-col justify-center">
            <div class="w-6 h-6 bg-blue-500 rounded-full"></div>
            <div class="w-12 h-8 bg-blue-500 rounded-t-full"></div>
        </div>
    {/if}
{/if}
