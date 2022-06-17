<script lang="ts">
  async function createGame(): Promise<String> {
    const token = await fetch('/api');
    console.log(await token.text());

    const game = await fetch('/api/create');
    const link = `/play?game_id=${await game.json()}`;
    if (game.ok) {
      return link;
    } else {
      throw new Error(":(");
    }
  }

  const link = createGame();
</script>

{#await link}
    <span>...</span>
{:then href}
    <a {href}>Play</a>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
