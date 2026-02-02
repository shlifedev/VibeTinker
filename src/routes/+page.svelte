<script lang="ts">
  import { commands } from "$lib/bindings"

  let name = $state("World")
  let greeting = $state("")

  async function handleGreet() {
    greeting = await commands.greet(name)
  }
</script>

<div class="container">
  <h1>New App</h1>
  <input type="text" bind:value={name} placeholder="Enter name" />
  <button onclick={handleGreet}>Greet</button>
  {#if greeting}
    <p>{greeting}</p>
  {/if}
</div>

<style>
  .container {
    padding: 2rem;
    text-align: center;
  }
  input {
    padding: 0.5rem;
    margin-right: 0.5rem;
  }
  button {
    padding: 0.5rem 1rem;
  }
</style>
