<script lang="ts">
  import Sidebar from '$lib/components/Sidebar.svelte';
  
  let { children } = $props();
</script>

<div class="app">
  <Sidebar />
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0a0a0f;
    color: #eee;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .app {
    display: flex;
    min-height: 100vh;
  }

  .content {
    margin-left: 240px;
    flex: 1;
    padding: 2rem;
  }
</style>
