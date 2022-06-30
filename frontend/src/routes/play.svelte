<script lang="ts">
  import {page} from '$app/stores';
  import Board from "../lib/Board.svelte";
  import Colour from "../lib/Board.svelte";
  import MovePiece from "../lib/Board.svelte";

  function onMessage(ev: MessageEvent) {
    console.log(`received: ${ev.data}`);
    const data = JSON.parse(ev.data);
  }

  async function handleMove(event) {
    const game = await fetch(`/api/submit/${$page.url.searchParams.get('game_id')}/${event.detail.text}`);
    const response = await game.text();
    alert(response)
    if(response === "Prima.") {
      alert("Prima!")
    } else {
      alert(":(")
    }
  }
</script>

<Board on:move={handleMove} player={Colour.Red}/>
