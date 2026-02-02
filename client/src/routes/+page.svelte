<script lang="ts">
  import { tools } from '$lib/config/tools';
</script>

<svelte:head>
  <title>Dev Tools</title>
</svelte:head>

<h1>Developer Productivity Tools</h1>
<p>Select a tool from the sidebar to get started.</p>

<div class="tools-grid">
  {#each tools as tool}
    <a href="/tools/{tool.slug}" class="tool-card">
      <span class="icon">{tool.icon}</span>
      <h2>{tool.name}</h2>
      <p>{tool.description}</p>
    </a>
  {/each}
</div>

<style>
  h1 {
    margin-bottom: 0.5rem;
  }

  .tools-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .tool-card {
    background: #16213e;
    border-radius: 8px;
    padding: 1.5rem;
    text-decoration: none;
    color: inherit;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .tool-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .tool-card .icon {
    font-size: 2rem;
  }

  .tool-card h2 {
    margin: 0.5rem 0;
    font-size: 1.25rem;
  }

  .tool-card p {
    margin: 0;
    color: #aaa;
    font-size: 0.9rem;
  }
</style>
