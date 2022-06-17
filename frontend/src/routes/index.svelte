<script lang="ts">
  import Card from "../lib/Card.svelte";
  import {onMount} from "svelte";

  async function getCard(): Promise<Card> {
    const res = await fetch(`/api/card`);
    const json = await res.json();
    if (res.ok) {
      return json;
    } else {
      throw new Error(":(");
    }
  }

  type Card = {
    animal: string;
    moves: Array<Move>;
  };

  type Move = {
    dx: number;
    dy: number;
  };

  const card = getCard();

  // onMount(async () => {
  //
  // });
</script>

{#await card}
    <p>...</p>
{:then card}
    <Card {...card}/>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
