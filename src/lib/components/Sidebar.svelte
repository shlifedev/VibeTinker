<script lang="ts">
  import { page } from "$app/stores"
  import { toolCategories, type ToolCategory, type Tool } from "$lib/config/tools"

  let expandedCategories = $state<Set<string>>(new Set(toolCategories.map((c: ToolCategory) => c.id)))

  function toggleCategory(categoryId: string) {
    if (expandedCategories.has(categoryId)) {
      expandedCategories.delete(categoryId)
    } else {
      expandedCategories.add(categoryId)
    }
    expandedCategories = new Set(expandedCategories)
  }

  function isActive(path: string): boolean {
    return $page.url.pathname === path
  }
</script>

<nav class="sidebar">
  <div class="sidebar-header">
    <h1>Dev Tools</h1>
  </div>
  <ul class="tree">
    {#each toolCategories as category}
      <li class="category">
        <button
          class="category-toggle"
          onclick={() => toggleCategory(category.id)}
        >
          <span class="arrow" class:expanded={expandedCategories.has(category.id)}>▶</span>
          {category.name}
        </button>
        {#if expandedCategories.has(category.id)}
          <ul class="tools">
            {#each category.tools as tool}
              <li>
                <a
                  href={tool.path}
                  class:active={isActive(tool.path)}
                >
                  {tool.name}
                </a>
              </li>
            {/each}
          </ul>
        {/if}
      </li>
    {/each}
  </ul>
</nav>

<style>
  .sidebar {
    width: 220px;
    height: 100vh;
    background: #1e1e1e;
    color: #e0e0e0;
    display: flex;
    flex-direction: column;
    border-right: 1px solid #333;
  }

  .sidebar-header {
    padding: 1rem;
    border-bottom: 1px solid #333;
  }

  .sidebar-header h1 {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 600;
  }

  .tree {
    list-style: none;
    padding: 0.5rem 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }

  .category {
    margin: 0;
  }

  .category-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.5rem 1rem;
    background: none;
    border: none;
    color: #e0e0e0;
    font-size: 0.9rem;
    cursor: pointer;
    text-align: left;
  }

  .category-toggle:hover {
    background: #2a2a2a;
  }

  .arrow {
    font-size: 0.7rem;
    transition: transform 0.2s;
  }

  .arrow.expanded {
    transform: rotate(90deg);
  }

  .tools {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .tools li a {
    display: block;
    padding: 0.4rem 1rem 0.4rem 2.2rem;
    color: #b0b0b0;
    text-decoration: none;
    font-size: 0.85rem;
  }

  .tools li a:hover {
    background: #2a2a2a;
    color: #e0e0e0;
  }

  .tools li a.active {
    background: #0e639c;
    color: #fff;
  }
</style>
