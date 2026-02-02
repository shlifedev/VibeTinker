<script lang="ts">
  import { page } from '$app/stores';
  import { tools } from '$lib/config/tools';
</script>

<aside class="sidebar">
  <div class="logo">
    <a href="/">Dev Tools</a>
  </div>
  
  <nav>
    <ul>
      {#each tools as tool}
        <li>
          <a 
            href="/tools/{tool.slug}" 
            class:active={$page.url.pathname === `/tools/${tool.slug}`}
          >
            <span class="icon">{tool.icon}</span>
            <span class="name">{tool.name}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>
</aside>

<style>
  .sidebar {
    width: 240px;
    height: 100vh;
    background: #1a1a2e;
    color: #eee;
    display: flex;
    flex-direction: column;
    position: fixed;
    left: 0;
    top: 0;
  }

  .logo {
    padding: 1.5rem;
    font-size: 1.25rem;
    font-weight: bold;
    border-bottom: 1px solid #333;
  }

  .logo a {
    color: inherit;
    text-decoration: none;
  }

  nav ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  nav li a {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    color: #aaa;
    text-decoration: none;
    transition: background 0.2s, color 0.2s;
  }

  nav li a:hover {
    background: #16213e;
    color: #fff;
  }

  nav li a.active {
    background: #0f3460;
    color: #fff;
  }

  .icon {
    font-size: 1.25rem;
  }
</style>
