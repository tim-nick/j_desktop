<script lang="ts">
  import { onMount } from 'svelte';
  import { remark } from 'remark';
  import remarkGfm from 'remark-gfm';
  import remarkHtml from 'remark-html';

  export let content: string;
  let htmlContent: string;

  onMount(async () => {
    const result = await remark()
      .use(remarkGfm)
      .use(remarkHtml)
      .process(content);
    htmlContent = result.toString();
  });
</script>

{@html htmlContent}