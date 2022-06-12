<script lang="ts">
  export let animal;
  export let moves;
</script>

<div class="absolute bg-slate-600 p-5 rounded-2xl border-8 border-slate-400 drop-shadow-lg flex flex-row items-center">
    <table class="border-4 border-slate-400">
        <tr>
            {#if moves.some(m => m.dx === -2 && m.dy === -2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === -1 && m.dy === -2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 0 && m.dy === -2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 1 && m.dy === -2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 2 && m.dy === -2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
        </tr>
        <tr>
            {#if moves.some(m => m.dx === -2 && m.dy === -1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === -1 && m.dy === -1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 0 && m.dy === -1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 1 && m.dy === -1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 2 && m.dy === -1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
        </tr>
        <tr>
            {#if moves.some(m => m.dx === -2 && m.dy === 0)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === -1 && m.dy === 0)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            <td class="w-5 h-5 border-2 border-slate-400 bg-slate-400"></td>
            {#if moves.some(m => m.dx === 1 && m.dy === 0)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 2 && m.dy === 0)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
        </tr>
        <tr>
            {#if moves.some(m => m.dx === -2 && m.dy === 1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === -1 && m.dy === 1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 0 && m.dy === 1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 1 && m.dy === 1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 2 && m.dy === 1)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
        </tr>
        <tr>
            {#if moves.some(m => m.dx === -2 && m.dy === 2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === -1 && m.dy === 2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 0 && m.dy === 2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 1 && m.dy === 2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
            {#if moves.some(m => m.dx === 2 && m.dy === 2)}
                <td class="w-5 h-5 border-2 border-slate-400 bg-pink-500"></td>
            {:else}
                <td class="w-5 h-5 border-2 border-slate-400"></td>
            {/if}
        </tr>
    </table>
    <span class="w-20 ml-5 text-2xl text-center text-slate-400 font-bold">{animal}</span>
</div>
