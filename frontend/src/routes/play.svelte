<script lang="ts">
  import {onMount} from "svelte";
  import { page } from '$app/stores'

  onMount(async () => {
    const socket = new WebSocket(`ws://${document.location.host}/api/connect/${$page.url.searchParams.get('game_id')}`);
    socket.onopen = (_: Event) => console.log('connected');
    socket.onmessage = onMessage;
  //  socket.onerror = (socket:WebSocket, ev: Event) => console.log(`connection error: ${ev.error}`);
    socket.onclose = (ev: CloseEvent) => console.log(`lost connection: ${ev.reason}`);
  });

  function onMessage(ev: MessageEvent) {
    console.log(`received: ${ev.data}`);
    const data = JSON.parse(ev.data);
  }

  type Card = {
    animal: string;
    moves: Array<Move>;
  };

  type Move = {
    dx: number;
    dy: number;
  };
</script>
