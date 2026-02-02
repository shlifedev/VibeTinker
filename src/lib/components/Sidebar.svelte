<script lang="ts">
  import { page } from "$app/stores"
  import { toolCategories, type ToolCategory } from "$lib/config/tools"
  import { Navigation } from "@skeletonlabs/skeleton-svelte"

  let expandedCategories = $state<Set<string>>(new Set(toolCategories.map((c: ToolCategory) => c.id)))

  function toggleCategory(categoryId: string) {
    if (expandedCategories.has(categoryId)) {
      expandedCategories.delete(categoryId)
    } else {
      expandedCategories.add(categoryId)
    }
    expandedCategories = new Set(expandedCategories)
  }
</script>

<Navigation layout="sidebar" class="h-screen border-r border-surface-200-800">
  <Navigation.Header>
    <a href="/" class="btn text-lg font-bold w-full justify-start px-4">Dev Tools</a>
  </Navigation.Header>
  <Navigation.Content>
    {#each toolCategories as category}
      <Navigation.Group>
        <Navigation.Label>
          <button
            class="btn w-full justify-start text-sm opacity-70 hover:opacity-100"
            onclick={() => toggleCategory(category.id)}
          >
            <span class="text-xs transition-transform" class:rotate-90={expandedCategories.has(category.id)}>▶</span>
            {category.name}
          </button>
        </Navigation.Label>
        {#if expandedCategories.has(category.id)}
          <Navigation.Menu>
            {#each category.tools as tool}
              <Navigation.TriggerAnchor
                href={tool.path}
                class={$page.url.pathname === tool.path ? "preset-filled-primary-500" : ""}
              >
                <Navigation.TriggerText>{tool.name}</Navigation.TriggerText>
              </Navigation.TriggerAnchor>
            {/each}
          </Navigation.Menu>
        {/if}
      </Navigation.Group>
    {/each}
  </Navigation.Content>
</Navigation>
