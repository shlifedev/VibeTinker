/**
 * Tool registry - add/remove tools here
 * 
 * To add a new tool:
 * 1. Add entry to this array
 * 2. Create route at /src/routes/tools/[slug]/+page.svelte
 */

export interface Tool {
  slug: string;
  name: string;
  description: string;
  icon: string; // emoji or icon class
}

export const tools: Tool[] = [
  {
    slug: 'base64',
    name: 'Base64',
    description: 'Encode and decode Base64 strings',
    icon: '🔐',
  },
  // Add more tools here
];

export function getToolBySlug(slug: string): Tool | undefined {
  return tools.find((t) => t.slug === slug);
}
