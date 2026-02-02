<script lang="ts">
  import "../app.css"
  import Sidebar from "$lib/components/Sidebar.svelte"
</script>

<div class="grid grid-cols-[auto_1fr] h-screen">
  <Sidebar />
  <main class="overflow-y-auto">
    <slot />
  </main>
</div>
