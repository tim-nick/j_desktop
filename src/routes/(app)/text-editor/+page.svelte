<script lang="ts">
	import EditorJsEditor from '$lib/components/text-editor/EditorJsEditor.svelte';
	// Optionally use your other Editor component as needed
	import Editor from '$lib/components/text-editor/Editor.svelte';

	const markdownContent = `
# Hello, GitHub Markdown!

This is a **bold** text and this is *italic*.
## This is h2

- List item 1
- List item 2
  - nested list 

\`\`\`javascript
console.log('Code block');
\`\`\`
  `;
</script>

<!-- Render the main editor component -->
<EditorJsEditor />
<!-- Or switch to the other editor if necessary -->
<!-- <Editor content={markdownContent}/> -->

<!-- <h1>Hello Editor </h1> -->
<!-- 
<style>
  :global(h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 20px 0;
  }
</style> -->