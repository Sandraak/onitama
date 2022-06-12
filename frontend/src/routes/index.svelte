<script lang="ts">
  import Card from "../lib/Card.svelte";

  type Card = {
    animal: string;
    moves: Array<Move>;
  };

  type Move = {
    dx: number;
    dy: number;
  };

  const response: Card = {animal: "Cobra", moves: [{dx: 1, dy: -1}, {dx: -1, dy: 0}, {dx: 1, dy: 1}]};
  const json = JSON.stringify(response);

  const card: Card = JSON.parse(json);
</script>

<Card {...card}/>
